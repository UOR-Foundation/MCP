// CF-04: the transcript the recorder writes holds every message, and says so when it cannot read one.
import assert from 'node:assert/strict';
import test from 'node:test';
import { eventData, exchanges, transcript } from '../recorder.mjs';

const connection = (request, response) => ({
  toServer: [Buffer.from(request, 'latin1')],
  toClient: [Buffer.from(response, 'latin1')],
  failed: null,
});

const post = (body, extra = '') =>
  `POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\n${extra}Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`;
const reply = (status, type, body) =>
  `HTTP/1.1 ${status} X\r\nContent-Type: ${type}\r\nContent-Length: ${Buffer.byteLength(body)}\r\nConnection: close\r\n\r\n${body}`;

test('a JSON exchange is a request and a response', () => {
  const list = exchanges([connection(post('{"id":1,"method":"m"}'), reply(200, 'application/json', '{"id":1,"result":{}}'))]);
  const out = transcript(list);
  assert.equal(out.version, 'prismpm/mcp-transcript/1');
  assert.deepEqual(out.messages.map((entry) => [entry.direction, entry.exchange]), [['client-to-server', 0], ['server-to-client', 0]]);
  assert.deepEqual(out.unread, []);
});

test('an event stream is one message per event, in order', () => {
  const stream = 'data: {"method":"notifications/progress"}\n\ndata: {"id":1,"result":{}}\n\n';
  const out = transcript(exchanges([connection(post('{"id":1,"method":"m"}'), reply(200, 'text/event-stream', stream))]));
  assert.deepEqual(out.messages.filter((entry) => entry.direction === 'server-to-client').map((entry) => entry.message.method ?? 'result'), ['notifications/progress', 'result']);
});

test('chunked bodies are decoded', () => {
  const body = '{"id":2,"method":"m"}';
  const chunked = `POST /mcp HTTP/1.1\r\nHost: x\r\nTransfer-Encoding: chunked\r\n\r\n${body.length.toString(16)}\r\n${body}\r\n0\r\n\r\n`;
  const out = transcript(exchanges([connection(chunked, reply(202, 'application/json', ''))]));
  assert.equal(out.messages[0].message.id, 2);
});

test('a body that is not JSON is listed as unread, never dropped', () => {
  const out = transcript(exchanges([connection(post('not json'), reply(403, 'text/plain; charset=utf-8', 'OriginForbidden\n'))]));
  assert.deepEqual(out.messages, []);
  assert.deepEqual(out.unread.map((entry) => entry.side), ['request', 'response']);
});

test('a connection with no response is listed as unread', () => {
  const out = transcript(exchanges([{ toServer: [Buffer.from(post('{"id":3,"method":"m"}'))], toClient: [], failed: 'server: ECONNRESET' }]));
  assert.equal(out.messages.length, 1);
  assert.deepEqual(out.unread, [{ exchange: 0, side: 'response', http: null, reason: 'server: ECONNRESET' }]);
});

test('event data follows the WHATWG reading', () => {
  assert.deepEqual(eventData('﻿: comment\r\ndata:a\r\ndata: b\r\n\r\nevent: x\n\ndata: c\n\n'), ['a\nb', 'c']);
});
