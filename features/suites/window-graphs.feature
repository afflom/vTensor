Feature: GGML window partition and join
  @VT-25 @build
  Scenario: Prism models window partition padding and window join logical indexing and Verdant transfers match the imported GGML CPU oracle for independent strided empty and bounded window inputs
    Given multiple window sizes, independent source windows and exact minimum join storage
    When Prism plans partition padding and bounded join indices through Verdant transfers
    Then complete outputs match the imported CPU graph across padded strided and empty layouts
    And short join storage and invalid parameters, extents and source types fail before writes
