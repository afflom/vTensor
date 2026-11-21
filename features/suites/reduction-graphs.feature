Feature: GGML reduction graphs
  @VT-29 @build
  Scenario: Prism models SUM SUM_ROWS and MEAN accumulation precision rounding and empty reductions and Verdant matches every supported float type against the imported GGML CPU oracle
    Given float tensors with cancellation signed zeros NaNs overflow and every empty axis
    When Prism orders row and global accumulation with the imported rounding rules
    Then each supported reduction matches the authoritative CPU graph and preserves padding
    And invalid shapes types and allocations are refused before writes
