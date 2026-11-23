Feature: verdant-api

  @VT-02 @build
  Scenario: The backend resolves the pinned Verdant API with fifteen ordered kernel descriptors and imports only GGML host services
    Given the compiled provider and the locked Verdant dependency
    When the oracle reads the kernel catalog and the shared library imports
    Then The backend resolves the pinned Verdant API with fifteen ordered kernel descriptors and imports only GGML host services

  @VT-03 @build
  Scenario: vTensor delegates fixed 4x4 F64 matrix multiplication to Verdant Tensor and preserves column-major results
    Given the fixture VT-03 needs
    When the suite exercises VT-03
    Then vTensor delegates fixed 4x4 F64 matrix multiplication to Verdant Tensor and preserves column-major results
