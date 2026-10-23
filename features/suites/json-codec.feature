Feature: JSON text codec

  The LexLean codec reads and writes JSON text as bytes, against RFC 8259, RFC 3629 and the pinned JSONTestSuite corpus.

  @JC-01 @build
  Scenario: Unicode byte validation admits exactly the byte strings RFC 3629 Table 3-7 admits, and refuses an overlong form, a surrogate code point, a value above the last code point and a truncated sequence, naming the offset of the first byte the table does not admit, or the end of the input when a sequence is cut short.
    Given the byte strings of one, two, three and four bytes, exhaustive where the space allows and at every Table 3-7 boundary elsewhere
    When the validator judges each of them
    Then its verdict class equals that of the Rust standard library validator and its offset equals that of an independent Table 3-7 reference

  @JC-02 @build
  Scenario: The parser refuses every byte string that is not well-formed UTF-8, with the typed refusals invalidUtf8 or truncatedUtf8 and an offset, before it reads any JSON structure.
    Given a byte string that is not well-formed UTF-8, wrapped in otherwise valid JSON
    When the parser reads it
    Then it returns invalidUtf8 or truncatedUtf8 with the offset and no JSON value

  @JC-03 @build
  Scenario: The parser accepts every text of the RFC 8259 grammar that the pinned JSONTestSuite lists as must-accept (the y_ cases).
    Given the 95 y_ files of the pinned JSONTestSuite
    When the parser reads each file
    Then each is accepted

  @JC-04 @build
  Scenario: The parser refuses every text that the pinned JSONTestSuite lists as must-reject (the n_ cases) with a typed refusal, and each case's refusal kind equals the recorded table.
    Given the 188 n_ files of the pinned JSONTestSuite
    When the parser reads each file
    Then each is refused with the refusal kind the recorded table names

  @JC-05 @build
  Scenario: The parser's outcome on every implementation-defined case of the pinned JSONTestSuite (the i_ cases) equals the recorded table.
    Given the 35 i_ files of the pinned JSONTestSuite
    When the parser reads each file
    Then each outcome equals the recorded table

  @JC-06 @build
  Scenario: String escapes decode as RFC 8259 section 7 states, an escaped surrogate pair is one code point, a lone surrogate escape is refused as loneSurrogate, and an unescaped control character is refused.
    Given strings with every short escape, a four-digit escape, a surrogate pair, a lone surrogate and a raw control character
    When the parser reads them
    Then escapes decode, a pair is one code point, a lone surrogate is refused as loneSurrogate and a raw control character is refused as controlInString

  @JC-07 @build
  Scenario: The RFC 8259 section 6 number grammar maps to a Decimal exactly, and a number with more than 18 significant digits or an effective exponent outside -9999..9999 is refused as numberOutOfRange.
    Given numbers at and beyond the declared bounds and the grammar's malformed forms
    When the parser reads them
    Then a well-formed number inside the bounds maps to its Decimal, a malformed one is refused as badNumber and one outside the bounds as numberOutOfRange

  @JC-08 @build
  Scenario: Nesting of arrays and objects to depth 512 is accepted and depth 513 is refused as tooDeep, without exhausting the stack.
    Given arrays and objects nested 512 and 513 deep
    When the parser reads them
    Then depth 512 is accepted and depth 513 is refused as tooDeep

  @JC-09 @build
  Scenario: Duplicate member names are kept in source order, so the parsed object has every member and Json.lookup answers with the first.
    Given an object text that names one member twice
    When the parser reads it and the object is looked up
    Then both members are kept in source order and the lookup answers with the first

  @JC-10 @build
  Scenario: The encoder writes compact RFC 8259 text, escapes quotation mark, reverse solidus and control characters, and refuses a number or nesting outside the declared bounds with encodeNumberOutOfRange or encodeTooDeep.
    Given values with strings that need escapes, and values outside the bounds
    When the encoder writes them
    Then the text is compact and escaped, and an out-of-bounds value is refused as encodeNumberOutOfRange or encodeTooDeep

  @JC-11 @build
  Scenario: For every text of the pinned JSONTestSuite the parser accepts, encoding the parsed value and parsing the result gives the same canonical bytes; the universal statement over all values is not claimed here.
    Given every accepted file of the pinned JSONTestSuite
    When its parsed value is encoded, parsed again and encoded again
    Then the second encoding equals the first

  @JC-12 @build
  Scenario: The oracle executes the package generated from the LexLean codec root, whose recorded digest and provenance match the committed files, over the corpus whose pin and checksum match the recorded authority.
    Given the committed generated package and the vendored corpus
    When their recorded digests are recomputed
    Then they equal the digests in the model and the pinned commit is the one named

  @JC-13 @build
  Scenario: Reading a JSON string, as a value or as a member name, takes time at most proportional to its length up to a logarithmic factor: at 256 KiB, 1 MiB and 2 MiB, through the codec root and through the lifecycle root, the read finishes within a bound linear in the length and its time grows no more than twice as fast as the length does.
    Given texts that hold one string, plain or with escapes and multi-byte characters, or one member name, of 256 KiB, 1 MiB and 2 MiB
    When each is read through the exported codec root and the exported lifecycle root
    Then each read finishes within the stated linear bound, its canonical text is the expected one, and the time grows no more than twice as fast as the length

  @JC-14 @build
  Scenario: The Unicode byte validator accepts every well-formed complete three-byte and four-byte sequence and refuses every ill-formed one as RFC 3629 and Table 3-7 state, naming the first byte the table does not admit, by kernel-checked theorems over all byte values with exact axiom policies.
    Given the lead-byte classes of Table 3-7 for three-byte and four-byte sequences
    When the theorems of the Utf8 module are checked by the kernel and the validator judges every such sequence
    Then the validator accepts exactly the well-formed ones, refuses each ill-formed one at the byte the table first refuses, and agrees with the standard library on every sequence
