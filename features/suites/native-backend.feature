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

  @VT-73 @build
  Scenario: Original GGML outer-product graphs with CPU-refused type and batch combinations match the authoritative decoder followed by the F32 CPU outer product
    Given the original factory operands for the CPU-refused outer-product cases
    When the plugin executes each typed graph and the imported evaluator decodes the same inputs for an F32 CPU contraction
    Then the plugin output satisfies the upstream comparison tolerance

  @VT-74 @build
  Scenario: Prism maps direct two-dimensional GGML convolution to padded image patches and kernel strides and Verdant contracts each patch matrix
    Given original GGML convolution graphs with F32 images and F16 or F32 kernels
    When vTensor gathers a tile of patches and Verdant multiplies it by the filter matrix
    Then every result matches the original GGML CPU evaluator

  @VT-75 @build
  Scenario: Prism admits GELU and GELU_QUICK GGML graphs and vTensor delegates the Gaussian activations to Verdant
    Given the pinned GGML unary and composed Gaussian activation graphs
    When both the vTensor plugin and reference CPU evaluate the same inputs
    Then all 64 original outputs satisfy the upstream case tolerances

  @VT-76 @build
  Scenario: Original channel-major GGML convolution graphs agree with upstream CPU convolution on equivalent contiguous tensors
    Given an original channel-major factory graph declined by the CPU kernel
    When GGML CPU independently materializes both operands from the same initialized leaves
    Then the original vTensor output agrees with the CPU convolution

  @VT-77 @build
  Scenario: Prism plans combined split and swapped gated rows and Verdant evaluates their rectified and Gaussian activations
    Given the upstream REGLU GEGLU and GEGLU_QUICK factory cases
    When the vTensor plugin and the pinned CPU process their original rows
    Then every output satisfies the original case tolerance

  @VT-78 @build
  Scenario: Prism binds head-specific masked attention and Verdant evaluates linear bias slopes before the weighted mean
    Given original attention graphs with head-specific mask scaling
    When the GGML CPU and vTensor process the same query keys values and masks
    Then each output matches the upstream comparator and tolerance

  @VT-79 @build
  Scenario: Prism maps one- and two-dimensional pooling windows and binds their maximum and average reductions to Verdant
    Given original GGML pooling graphs with strided windows and padding
    When vTensor executes the windows and their dimension-specific padding rules
    Then every result matches the original GGML CPU evaluator

  @VT-80 @build
  Scenario: Prism maps direct three-dimensional GGML convolution to padded volume patches and packed channel offsets and Verdant contracts each patch matrix
    Given original GGML convolution graphs with F32 images and F16 or F32 kernels
    When vTensor gathers a tile of patches and Verdant multiplies it by the filter matrix
    Then every result matches the original GGML CPU evaluator

  @VT-81 @build
  Scenario: Prism maps each state convolution window to its channel and Verdant accumulates ordered binary32 products
    Given original state convolution graphs with batched F32 inputs and optional channel bias and SiLU
    When vTensor maps each sequence and channel to a sliding window
    Then every result matches the original GGML CPU evaluator

  @VT-82 @build
  Scenario: Prism gathers contributing columns in input order and Verdant sums their stored values in binary32
    Given original GGML column-to-signal graphs with F32, F16 and BF16 storage
    When vTensor reconstructs strided overlapping or cropped windows
    Then the result for every output channel matches the imported GGML CPU evaluator

  @VT-83 @build
  Scenario: Prism selects expert rows and Verdant evaluates packed indexed matrix products
    Given original GGML expert matrices, F32 activations and strided I32 indices
    When vTensor executes packed matrix products with repeated, broadcast or skipped experts
    Then its outputs agree with the pinned GGML CPU evaluator

  @VT-84 @build
  Scenario: Prism selects scalar expert rows and activation precision and Verdant multiplies the selected rows
    Given original indexed matrix graphs with F32, F16 or BF16 expert weights
    When vTensor executes the selected experts over broadcast activations and offset index views
    Then each output matches the original GGML CPU evaluator

  @VT-85 @build
  Scenario: Prism schedules ordinary and indexed E4M3 weight products and Verdant decodes the weights and contracts binary32 rows
    Given original GGML matrix and expert graphs with E4M3 weights
    When vTensor executes the selected rows with F32, F16 or BF16 activation storage
    Then the original CPU evaluator accepts every output

  @VT-86 @build
  Scenario: Prism schedules binary32 row ranking and Verdant selects deterministic indices for GGML ARGSORT and TOP_K
    Given original GGML rows with distinct or tied values
    When vTensor orders the rows or selects their largest entries
    Then the imported CPU evaluator accepts the returned indices

  @VT-59 @build
  Scenario: The vTensor CPU registry creates attaches switches and releases threadpools while GGML graphs match the imported CPU across repeated submissions
    Given the CPU registry function names used by the pinned llama.cpp host
    When an application creates two pools and attaches, switches and detaches them
    Then the submitted graph outputs match the imported CPU through pool reuse and teardown

  @VT-87 @build
  Scenario: Prism binds GGML masks, sinks and row layouts and Verdant normalizes the score rows against the imported CPU softmax
    Given original GGML score rows with optional masks and attention sinks
    When vTensor normalizes the rows including in-place destinations
    Then the imported CPU evaluator accepts the output probabilities

  @VT-88 @build
  Scenario: Prism plans ordinary and indexed Q4_K and Q5_K weight products and Verdant contracts the packed weights against grouped signed activations
    Given original GGML four- and five-bit grouped weight graphs
    When the provider computes matrix rows or selects experts
    Then the independent CPU backend accepts the computed outputs

  @VT-89 @build
  Scenario: Prism binds GGML softplus storage and views and Verdant computes the binary32 activation against the pinned CPU
    Given original GGML binary32, binary16 and bfloat16 activation views
    When vTensor applies softplus to each input
    Then the imported CPU evaluator accepts the computed activation

  @VT-91 @build
  Scenario: Prism plans ordinary and indexed Q1_0 and Q2_0 weight products and Verdant contracts the packed weights against symmetric eight-bit activation blocks
    Given original GGML one- and two-bit packed weight graphs
    When the provider computes matrix rows or selects experts
    Then the independent CPU backend accepts the computed outputs
