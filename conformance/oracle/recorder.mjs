// A recording TCP relay between the conformance suite and the served core.
//
// It copies bytes in both directions and changes none, so the host adapter sees
// exactly the request the suite wrote (field order, case and Host included). Each
// connection is kept as the bytes of both directions. `exchanges` reads them as
// HTTP/1.1 messages afterwards: the relay itself decides nothing about them.
import net from 'node:net';

export function startRecorder(upstreamPort) {
  const connections = [];
  const server = net.createServer((client) => {
    const record = { opened: connections.length, toServer: [], toClient: [], failed: null };
    connections.push(record);
    const upstream = net.connect({ port: upstreamPort, host: '127.0.0.1' });
    const fail = (side) => (error) => {
      record.failed ??= `${side}: ${error.code ?? error.message}`;
      client.destroy();
      upstream.destroy();
    };
    client.on('data', (chunk) => {
      record.toServer.push(chunk);
      upstream.write(chunk);
    });
    upstream.on('data', (chunk) => {
      record.toClient.push(chunk);
      client.write(chunk);
    });
    client.on('end', () => upstream.end());
    upstream.on('end', () => client.end());
    client.on('error', fail('client'));
    upstream.on('error', fail('server'));
    client.on('close', () => upstream.destroy());
    upstream.on('close', () => client.destroy());
  });
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      resolve({
        port: server.address().port,
        connections,
        close: () => new Promise((done) => {
          server.close(() => done());
          server.closeAllConnections?.();
        }),
      });
    });
  });
}

const CRLF = Buffer.from('\r\n\r\n');

function split(buffer) {
  const end = buffer.indexOf(CRLF);
  if (end < 0) return null;
  const lines = buffer.subarray(0, end).toString('latin1').split('\r\n');
  const fields = lines.slice(1).map((line) => {
    const colon = line.indexOf(':');
    return colon < 0 ? [line, ''] : [line.slice(0, colon), line.slice(colon + 1).trim()];
  });
  return { start: lines[0], fields, rest: buffer.subarray(end + CRLF.length) };
}

function field(fields, name) {
  return fields.find(([key]) => key.toLowerCase() === name)?.[1];
}

function dechunk(buffer) {
  const parts = [];
  let at = 0;
  for (;;) {
    const eol = buffer.indexOf('\r\n', at, 'latin1');
    if (eol < 0) return null;
    const size = Number.parseInt(buffer.subarray(at, eol).toString('latin1').split(';')[0], 16);
    if (Number.isNaN(size)) return null;
    at = eol + 2;
    if (size === 0) return Buffer.concat(parts);
    if (buffer.length < at + size) return null;
    parts.push(buffer.subarray(at, at + size));
    at += size + 2;
  }
}

function body(fields, rest) {
  if (field(fields, 'transfer-encoding')?.toLowerCase() === 'chunked') return dechunk(rest) ?? rest;
  const length = Number.parseInt(field(fields, 'content-length') ?? '', 10);
  return Number.isNaN(length) ? rest : rest.subarray(0, length);
}

/** Read the recorded connections as HTTP/1.1 exchanges, in the order they opened. */
export function exchanges(connections) {
  const out = [];
  for (const [index, record] of connections.entries()) {
    const request = split(Buffer.concat(record.toServer));
    if (!request) continue;
    const [method, target] = request.start.split(' ');
    const reply = split(Buffer.concat(record.toClient));
    const exchange = {
      index,
      method,
      target,
      requestFields: request.fields,
      requestBody: body(request.fields, request.rest),
      status: null,
      responseFields: [],
      responseBody: Buffer.alloc(0),
      failed: record.failed,
    };
    if (reply) {
      exchange.status = Number.parseInt(reply.start.split(' ')[1], 10);
      exchange.responseFields = reply.fields;
      exchange.responseBody = body(reply.fields, reply.rest);
    }
    out.push(exchange);
  }
  return out;
}

/** The data of each event of a `text/event-stream` body, read as the WHATWG format does. */
export function eventData(text) {
  const events = [];
  let data = [];
  for (const line of text.replace(/^﻿/, '').split(/\r\n|\r|\n/)) {
    if (line === '') {
      if (data.length > 0) events.push(data.join('\n'));
      data = [];
    } else if (line.startsWith('data:')) {
      const value = line.slice(5);
      data.push(value.startsWith(' ') ? value.slice(1) : value);
    }
  }
  return events;
}

/**
 * The transcript of the exchanges: every JSON-RPC message the suite sent and every
 * one the server answered with, request before response. A body that is not JSON is
 * not dropped silently: it is listed in `unread`, with its exchange.
 */
export function transcript(list) {
  const messages = [];
  const unread = [];
  const note = (exchange, side, reason) => unread.push({ exchange: exchange.index, side, http: exchange.status, reason });
  for (const exchange of list) {
    const base = { exchange: exchange.index, http: `${exchange.method} ${exchange.status ?? 'no response'}` };
    const push = (direction, message) => messages.push({ direction, message, ...base });
    if (exchange.requestBody.length > 0) {
      try {
        push('client-to-server', JSON.parse(exchange.requestBody.toString('utf8')));
      } catch {
        note(exchange, 'request', 'the body is not JSON');
      }
    }
    if (exchange.status === null) {
      note(exchange, 'response', exchange.failed ?? 'the server sent no response');
      continue;
    }
    const type = (exchange.responseFields.find(([key]) => key.toLowerCase() === 'content-type')?.[1] ?? '').toLowerCase();
    const text = exchange.responseBody.toString('utf8');
    if (exchange.responseBody.length === 0) continue;
    const texts = type.startsWith('text/event-stream') ? eventData(text) : [text];
    for (const piece of texts) {
      try {
        push('server-to-client', JSON.parse(piece));
      } catch {
        note(exchange, 'response', `the body is not JSON (${type || 'no content type'})`);
      }
    }
  }
  return { version: 'prismpm/mcp-transcript/1', spec: '2026-07-28', messages, unread };
}
