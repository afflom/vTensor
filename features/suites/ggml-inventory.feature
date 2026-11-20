Feature: ggml-inventory

  @VT-04 @build
  Scenario: The imported GGML operation and tensor type declarations are inventoried exactly without implementation claims
    Given the GGML header and its upstream revision are pinned
    When the declaration inventory is compared with both authoritative enums
    Then every declaration preserves upstream order and numeric value including gaps and sentinels
    And omissions additions numeric drift and implementation claims are rejected
