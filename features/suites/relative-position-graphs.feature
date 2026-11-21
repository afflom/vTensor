Feature: GGML relative-position word selection
  @VT-33 @build
  Scenario: Prism models GGML relative-position word selection and Verdant preserves F16 and BF16 source encodings in F16 outputs across strided empty and higher-dimensional source layouts
    Given F16 and BF16 words including exceptional encodings, multiple relative widths and higher source dimensions
    When Prism selects relative-position source words and Verdant transfers their exact bytes
    Then all outputs and padding match the imported GGML CPU graph for contiguous strided and empty layouts
    And malformed types dimensions strides aliases and missing storage are refused before writes
