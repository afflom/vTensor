Feature: Indexed row addition
  @VT-34 @build
  Scenario: Prism models three-source indexed row addition and Verdant matches GGML ADD_ID arithmetic and exact minus-one row copies across strides empty tensors and atomic invalid-index refusals
    Given strided F32 values and row tables with I32 indices including minus-one copies and ignored higher index slices
    When Prism preflights every consumed index and Verdant executes the admitted row additions and copies
    Then output words and padding match the imported GGML CPU oracle including exact exceptional-word copies
    And malformed descriptors aliases and late invalid indices are refused before output writes
