Feature: Certified graph fixtures
  @VT-35 @build
  Scenario: Prism generates all matrix fixture dimensions layouts and input words matching the complete frozen prior oracle corpus while GGML independently computes every expected result
    Given the complete frozen records of 1280 prior matrix cases including source layouts and exceptional input words
    When the certified Prism fixture model generates every case and the imported GGML constructors bind it
    Then generated and observed fixture records match the frozen corpus byte for byte
    And the imported GGML CPU computes every expected numerical result independently
    And changed source words missing cases and wrong field bindings are refused

  @VT-36 @build
  Scenario: Prism generates every arithmetic fixture descriptor and input word while preserving the complete frozen broadcasting and exceptional-word corpus and the independent GGML CPU numerical oracle
    Given all 120 frozen arithmetic cases across eight operations three widths padding and broadcasting
    When the certified fixture model supplies the actual imported GGML constructors
    Then every generated and observed descriptor and input word matches the prior corpus
    And the GGML CPU independently computes every expected result
    And source word changes and native binding defects are detected

  @VT-40 @build
  Scenario: Native fixture construction initialized bytes execution descriptors and atomic refusal mutations match the frozen pre-migration corpus while expected numerical results remain computed by the imported GGML CPU oracle
    Given sixteen native fixture families captured before their recipe migration
    When their existing CPU oracle tests execute
    Then graph descriptors aliases initialized bytes and refusal mutations match the frozen transcripts
    And missing stale truncated or altered transcripts are rejected
    And each case executes only once in the complete workspace run
    And copied FILL source-buffer release is frozen at the actual execution boundary

  @VT-41 @build
  Scenario: Prism generates COPY fixture enumeration source and target descriptors and initial bytes while preserving all 3456 frozen cases and independent GGML CPU expected results
    Given the frozen COPY corpus for all 36 live storage types
    When certified Prism recipes supply the native GGML constructors and initial byte blocks
    Then every construction input alias and refusal record remains byte identical
    And the unchanged CPU oracle computes every expected result
    And invalid recipe requests and altered recipe bindings are rejected
