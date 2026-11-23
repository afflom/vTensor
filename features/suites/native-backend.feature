Feature: native-backend

  @VT-11 @build
  Scenario: GGML discovers the Prism-defined vTensor device and its owned buffers preserve upstream allocation and byte-transfer behavior
    Given the pinned upstream GGML backend ABI and native frontend
    When an application discovers vTensor and allocates and transfers tensor bytes
    Then ABI layouts, tensor placement, and observed bytes agree with upstream
    And invalid allocation and byte ranges have modeled refusals
    And native oracle executables are reused after host-only orchestration changes
    And changes to the compiler recipe or imported evaluator invalidate that reuse

  @VT-52 @build
  Scenario: Prism selects graph nodes by the imported GGML compute flag and vTensor preserves inactive results across scheduled graph reuse
    Given a scheduled graph with an intermediate result and a dependent consumer
    When the application reuses the graph with the intermediate compute bit set or cleared
    Then both node outputs match the imported CPU and each compute node is assigned to the selected device

  @VT-60 @build
  Scenario: GGML logarithm sine cosine and exponential graphs execute through the Verdant scalar primitives across the imported factory storage and view cases
    Given the pinned upstream GGML evaluation factory and CPU reference
    When LOG, SIN, COS and EXP run on the vTensor backend
    Then all 39 factory outputs match the reference within the imported tolerances

  @VT-61 @build
  Scenario: GGML sigmoid and SiLU execute through Verdant in unary, multiplied and added graph compositions across all 122 imported factory cases
    Given the pinned GGML CPU reference and graph factory
    When vTensor evaluates sigmoid, SiLU and their add and multiply compositions
    Then the 122 results match the upstream evaluator

  @VT-62 @build
  Scenario: GGML BF16 row gradients accumulate into F32 with repeated and strided indices and match the pinned CPU widening and row-gradient operators
    Given the pinned GGML CPU cast and F32 GET_ROWS_BACK operators
    When vTensor accumulates encoded BF16 gradients into the requested F32 rows
    Then the output and unchanged padding match GGML and invalid indices are refused atomically

  @VT-63 @build
  Scenario: GGML attention with BF16 key and value rows uses Verdant conversion and contraction and matches the imported CPU with optional masks and sinks
    Given the pinned GGML BF16 attention factory with 64 channels and 113 keys
    When vTensor evaluates all four combinations of mask and sink presence
    Then each output matches the imported GGML CPU evaluator

  @VT-64 @build
  Scenario: GGML matrix products with F16 and BF16 operands honor the input dot-product storage and match the imported CPU across six scalar type pairs
    Given the pinned GGML matrix factory and reference CPU
    When vTensor executes dense and permuted matrix products with the six F16 or BF16 operand pairs
    Then all ten representative graphs and the high-range F32 case match the reference

  @VT-65 @build
  Scenario: GGML Q2_K matrix products use Verdant grouped activation blocks and match the imported CPU in standalone and row-gather graphs
    Given the pinned GGML Q2_K matrix factory and row-gather-after-product graph
    When vTensor executes each graph on its own device
    Then all 55 matrix cases and the composed ADD case match the upstream CPU

  @VT-66 @build
  Scenario: GGML extended attention with Q4_0 and Q8_0 keys and values executes through Verdant block conversion and row products and matches the imported CPU
    Given the original GGML attention factory with packed keys and values
    When vTensor executes each combination of mask and sink presence for the two formats
    Then all eight graphs match the GGML CPU evaluator

  @VT-67 @build
  Scenario: GGML hyperbolic tangent and composed soft-cap graphs execute through the pinned Verdant binary32 activation and match the imported CPU evaluator
    Given the pinned GGML factory for TANH and SOFTCAP
    When the backend executes scalar storage variants, views and composed scale operations
    Then every result matches the upstream CPU evaluator

  @VT-68 @build
  Scenario: Prism places attention logit soft caps before the mask and Verdant evaluates the scaled hyperbolic tangent against the imported GGML CPU
    Given the original F16 attention cases with a soft cap of ten
    When vTensor runs all four combinations of mask and sink presence
    Then every output passes the upstream CPU comparison

  @VT-69 @build
  Scenario: Prism maps GGML scalar outer-product axes and batch replication to the Verdant matrix primitive and matches the imported CPU evaluator
    Given the upstream outer-product factory with F32 activations and F32 or F16 weights
    When vTensor executes the CPU-supported scalar cases across row and batch shapes
    Then all 87 outputs match the reference evaluator

  @VT-70 @build
  Scenario: Prism maps GGML three-dimensional image columns through padded and dilated input views, with Verdant storing each typed output
    Given imported IM2COL_3D graphs with strided input views and F16 or F32 output
    When vTensor places each image sample or padding value in its result column
    Then every result matches the original GGML CPU evaluator

  @VT-71 @build
  Scenario: Prism maps packed GGML outer-product rows into strided Verdant block decoding and matrix contraction and matches the imported CPU evaluator
    Given all 144 CPU-supported outer-product cases with packed left operands
    When vTensor contracts each loaded row through the pinned Verdant kernels
    Then every output matches the upstream CPU reference

  @VT-72 @build
  Scenario: Prism maps one- and two-dimensional GGML image columns across channel and batch strides and Verdant converts the result storage
    Given imported IM2COL graphs with horizontal or planar kernels and F16 or F32 output
    When vTensor places each image sample or padding value in its result column
    Then every result matches the original GGML CPU evaluator

  @VT-74 @build
  Scenario: Prism maps direct two-dimensional GGML convolution to padded image patches and kernel strides and Verdant contracts each patch matrix
    Given original GGML convolution graphs with F32 images and F16 or F32 kernels
    When vTensor gathers a tile of patches and Verdant multiplies it by the filter matrix
    Then every result matches the original GGML CPU evaluator
