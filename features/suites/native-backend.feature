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

  @VT-90 @build
  Scenario: Prism binds F16 activation rows for packed matrix products and Verdant widens each activation before the certified dot product
    Given original GGML matrix graphs with packed weights and F16 activations
    When the loaded provider evaluates the graphs
    Then the results match pinned GGML widening and CPU matrix multiplication

  @VT-91 @build
  Scenario: Prism plans ordinary and indexed Q1_0 and Q2_0 weight products and Verdant contracts the packed weights against symmetric eight-bit activation blocks
    Given original GGML one- and two-bit packed weight graphs
    When the provider computes matrix rows or selects experts
    Then the independent CPU backend accepts the computed outputs

  @VT-92 @build
  Scenario: Prism plans sparse GGML top-k mask cells and Verdant compares scores and transfers selected mask words
    Given the original GGML sparse mask graphs with optional scores blocks and extra cells
    When the loaded provider computes each output row
    Then the selected original mask words and excluded cells match the independent CPU

  @VT-93 @build
  Scenario: Prism binds GGML clamp limits and storage for SwiGLU and Verdant evaluates the bounded operands in binary32
    Given the original GGML F32 F16 and BF16 SwiGLU graphs with two clamp limits
    When the loaded provider computes the bounded gate and projects each row
    Then the resulting values match the imported CPU precision for each storage type

  @VT-94 @build
  Scenario: Prism binds Q5_0 and IQ4_NL matrix and attention rows to Verdant signed-coefficient contractions and value decoding
    Given original ordinary matrix indexed expert and extended attention graphs
    When vTensor runs with signed five-bit or nonlinear four-bit rows
    Then the imported CPU accepts each numerical output

  @VT-95 @build
  Scenario: Prism binds affine four- and five-bit matrix and attention rows to Verdant activation encoding, row products and value decoding
    Given original ordinary matrix indexed expert and extended attention graphs
    When vTensor runs with affine four- or five-bit weight rows
    Then the imported CPU accepts each numerical output

  @VT-96 @build
  Scenario: Prism admits sign, two-bit and E4M3 attention rows and binds their query conversion, key contraction and value reads to Verdant
    Given the nineteen original GGML short-bit and E4M3 attention constructors
    When vTensor executes the original key, value and query layouts
    Then the imported CPU accepts each numerical result

  @VT-97 @build
  Scenario: Prism validates indexed key and value cache rows and maps each unmasked column to the selected cache cell for Verdant attention
    Given original GGML indexed key and value cache constructors across slice counts and storage types
    When vTensor executes the original per-slice cache indices and negative padding
    Then the imported CPU accepts each numerical result

  @VT-98 @build
  Scenario: Prism maps strided scalar copies between E4M3 and GGML float storage to the existing Verdant word conversions
    Given the original E4M3 copy graphs and their CPU conversion tables
    When vTensor executes the typed copies through the original views and strides
    Then the imported CPU accepts each numerical result

  @VT-99 @build
  Scenario: Prism plans MXFP4 and NVFP4 matrix and indexed expert rows and Verdant contracts their E2M1 weights with signed-byte activations
    Given the pinned GGML block-float matrix and expert constructors
    When vTensor runs the original graphs on the imported inputs
    Then the numerical rows match the independent GGML CPU evaluator

  @VT-100 @build
  Scenario: Prism maps indexed scalar row writes to E4M3 storage and Verdant performs the signed conversion after source widening
    Given the original F16 and F32 indexed row writes into E4M3 storage
    When vTensor executes the indexed writes through the original destination view
    Then the imported CPU accepts each numerical result

  @VT-101 @build
  Scenario: Prism selects scalar source lanes and destination blocks for copies and indexed row writes while Verdant encodes Q8_0
    Given the original GGML indexed row writes and scalar copies into symmetric byte blocks
    When vTensor gathers the source lanes and writes each encoded block through the original destination view
    Then the imported CPU accepts each numerical result

  @VT-102 @build
  Scenario: Prism plans indexed gradient accumulation by source block and Verdant decodes the source and adds each binary32 lane
    Given GGML gradient rows and a contiguous or viewed index vector
    When vTensor scatters the decoded rows into the binary32 gradient
    Then the output matches authoritative GGML decoding followed by CPU GET_ROWS_BACK

  @VT-103 @build
  Scenario: Prism plans grouped two-bit codebook matrix and expert graphs and Verdant contracts their signed codeword rows
    Given IQ2_XXS, IQ2_XS or IQ2_S weights and grouped activations
    When the provider evaluates ordinary and indexed matrix products
    Then the output matches the pinned GGML CPU evaluator

  @VT-104 @build
  Scenario: Prism addresses source blocks and binary32 destination lanes for type-changing copies and Verdant decodes each original GGML input
    Given a packed or signed integer GGML source and binary32 destination
    When vTensor copies contiguous or permuted source views
    Then the destination values match the imported CPU evaluator

  @VT-105 @build
  Scenario: Prism plans indexed and copied four- and five-bit block writes and Verdant encodes the original binary32 and half-width sources
    Given an indexed destination or a copied view of four- or five-bit blocks
    When the provider stores complete weight blocks
    Then the original GGML comparisons pass and late failures preserve destination bytes

  @VT-106 @build
  Scenario: Prism addresses one-dimensional transposed convolution taps and overlaps and Verdant contracts and sums the input channels
    Given a binary32 signal, kernel and stride
    When vTensor contracts the input channels and accumulates overlapping kernel taps
    Then the transposed convolution matches the pinned GGML CPU evaluator

  @VT-107 @build
  Scenario: Prism addresses two-dimensional transposed convolution taps and overlaps and Verdant contracts and sums the input channels
    Given a batched binary32 image, a floating-point kernel and a stride
    When vTensor contracts the input channels and accumulates overlapping kernel taps
    Then the transposed convolution matches the pinned GGML CPU evaluator

  @VT-109 @build
  Scenario: Prism plans full-width sign and centered two-bit writes and Verdant encodes every indexed or copied destination block
    Given copied or indexed rows of sign or centered two-bit coefficients
    When the provider encodes each destination block
    Then the output matches the original GGML CPU row writes

  @VT-110 @build
  Scenario: Prism plans signed three-bit and six-bit grouped matrix and expert graphs and Verdant contracts their quantized weight rows
    Given Q3_K or Q6_K weights and grouped activations
    When the provider evaluates ordinary and indexed matrix products
    Then each graph matches the pinned GGML CPU evaluator

  @VT-111 @build
  Scenario: Prism schedules parameterized SwiGLU clipping, exponential and arithmetic through Verdant for fused and separate inputs
    Given clipped activation and gate values in a GGML SwiGLU graph
    When the provider applies the supplied alpha and limit
    Then its output matches the imported GGML CPU graph

  @VT-112 @build
  Scenario: Prism plans grouped nonlinear four-bit ordinary and expert matrices and Verdant evaluates their row products
    Given grouped IQ4_XS weights and F32 activations
    When the provider executes an ordinary or indexed matrix product
    Then each output matches the pinned GGML CPU

  @VT-113 @build
  Scenario: Prism schedules 256-lane ternary row encoders for copied and indexed writes through Verdant
    Given TQ1_0 or TQ2_0 destinations and scalar source views
    When vTensor stages a copied or indexed row write
    Then the stored bytes match GGML CPU and a rejected index preserves the destination

  @VT-114 @build
  Scenario: Prism plans ordinary and expert matrices with ternary weights and Verdant contracts their rows
    Given TQ1_0 or TQ2_0 weights in ordinary and indexed matrix graphs
    When vTensor computes the imported graph
    Then each result passes the pinned GGML CPU comparison

  @VT-115 @build
  Scenario: Prism plans ordinary and expert matrices with four-lane codebooks and Verdant contracts their quantized rows
    Given IQ3_XXS or IQ3_S weight rows and F32 activations
    When the provider evaluates an ordinary or indexed matrix
    Then each result matches the imported GGML CPU

  @VT-116 @build
  Scenario: Prism plans ordinary and expert matrices with shifted one-bit codebooks and Verdant contracts their quantized rows
    Given IQ1_S or IQ1_M weight rows and F32 activations
    When the provider evaluates an ordinary or indexed matrix
    Then each result matches the imported GGML CPU

  @VT-117 @build
  Scenario: Prism schedules exponent-only and grouped four-bit floating encoders for copied and indexed GGML writes through Verdant
    Given MXFP4 or NVFP4 destinations with scalar source views
    When vTensor copies or updates indexed rows
    Then stored bytes match GGML CPU and a failed update preserves the destination

  @VT-118 @build
  Scenario: Prism composes Verdant exponential and binary32 subtraction for strided GGML EXPM1 graphs
    Given the imported exponential-minus-one graphs
    When the provider widens each input and subtracts one from its binary32 exponential
    Then each result agrees with the pinned CPU unary operation

  @VT-119 @build
  Scenario: Prism places grouped signed six-bit Verdant encodings through GGML indexed writes and copies
    Given Q6_K destinations with scalar source views
    When vTensor copies or updates indexed rows
    Then the stored groups agree with the CPU and a failed write preserves the destination

  @VT-120 @build
  Scenario: Prism admits ordinary and in-place E4M3 additions and Verdant reproduces the GGML CPU results
    Given all six imported E4M3 addition graphs with F32 right operands
    When the backend adds the widened operands and stores the result
    Then the original CPU graph agrees for ordinary and aliased outputs

  @VT-121 @build
  Scenario: Prism schedules the rowwise softmax derivative and Verdant reproduces the imported GGML gradient
    Given the imported binary32 softmax gradient graphs with zero and positive head bias
    When the backend reduces each probability-gradient dot product and applies the upstream scale
    Then the results agree with the pinned GGML CPU and Vulkan backends for their admitted graphs

  @VT-122 @build
  Scenario: Prism schedules grouped affine two-, four- and five-bit copies and indexed writes through the Verdant encoders
    Given original GGML copies and indexed writes to grouped affine destinations
    When vTensor places each encoded block in the requested row or view
    Then the upstream CPU comparison passes for all three storage formats and rejected indices preserve output

  @VT-123 @build
  Scenario: Prism admits GGML scheduler delays that preserve their input view and complete before graph return
    Given an original GGML sleep graph with an aliased input
    When the graph runs on the CPU oracle and vTensor
    Then both preserve the input bytes and wait for the requested duration

  @VT-124 @build
  Scenario: Prism schedules the binary32 SiLU derivative with Verdant arithmetic
    Given upstream binary32 activation values and independent incoming gradients
    When the backend multiplies each incoming gradient by the SiLU derivative
    Then the result agrees with the original GGML CPU graph

  @VT-125 @build
  Scenario: Prism schedules depthwise spatial convolution in both GGML channel layouts using Verdant products and sums
    Given the imported depthwise convolution graphs with F32 and F16 filters
    When the backend contracts each filter with its own channel at the requested stride, padding and dilation
    Then the outputs match the pinned GGML CPU in WHCN and CWHN storage

  @VT-126 @build
  Scenario: Prism stages binary32 to signed integer copies through the Verdant truncation operation
    Given the imported F32 to I32 copies with contiguous and permuted source storage
    When the backend converts all source elements before committing the destination
    Then the destination words match the pinned GGML CPU

  @VT-127 @build
  Scenario: Prism contracts hyperconnection channels through Verdant with optional per-element sigmoid gates
    Given the imported weighted and sigmoid-gated hyperconnection reductions
    When the backend reduces the channels for each embedding coordinate and token, then applies the output scale
    Then the outputs match the pinned GGML CPU for all channel counts

  @VT-128 @build
  Scenario: Dense and indexed projections preserve explicit GGML accumulator and activation precision across stored weight formats
    Given GGML projections with an explicit minimum activation and accumulation precision
    When their weights have scalar or packed storage
    Then Prism selects a Verdant product meeting both requested precisions

  @VT-108 @build
  Scenario: Prism selects query heads and key rows and Verdant computes masked weighted rectified attention scores
    Given queries, keys, head weights and a batch-broadcast half precision mask
    When vTensor decodes keys, contracts query heads and adds the masked scores
    Then the scores match the pinned GGML CPU evaluator

  @VT-129 @build
  Scenario: Prism models residual hyperconnection mixing and Verdant contracts optional channel combinations against the GGML CPU evaluator
    Given an activation, channel residuals, post weights and an optional combination tensor
    When vTensor evaluates identity or mixed residual channels
    Then each output matches the pinned GGML CPU graph

  @VT-130 @build
  Scenario: Prism schedules four-channel hyperconnection normalization and Verdant evaluates each word against the GGML CPU combination oracle
    Given four-channel mixture logits, a scale, base offsets, epsilon and a positive iteration count
    When vTensor performs row softmax and iterative column and row normalization
    Then each output matches the pinned GGML CPU graph

  @VT-131 @build
  Scenario: Prism schedules in-place SGD and AdamW updates and Verdant evaluates parameters and Adam moments against GGML CPU
    Given weights, gradients, optimizer parameters and the AdamW moment tensors
    When vTensor submits the optimizer graph repeatedly
    Then weights and persistent moment updates match the pinned GGML CPU graph

  @VT-132 @build
  Scenario: Prism models cross-entropy loss and its logit gradient through Verdant reductions against GGML
    Given rows of logits and target probabilities with an incoming loss gradient
    When vTensor computes the mean loss and its gradient across classes
    Then both results match the imported GGML CPU evaluator

  @VT-133 @build
  Scenario: Prism admits Q3_K copies and indexed writes and Verdant encodes each selected block against the GGML CPU reference
    Given floating point values and a grouped three-bit destination
    When GGML copies values or updates indexed rows
    Then the resulting tensor matches the imported CPU encoder

  @VT-134 @build
  Scenario: Prism schedules batched lower triangular solves and Verdant performs forward substitution against GGML CPU
    Given a lower triangular matrix and one or more right-hand columns in each batch
    When vTensor solves each row from the previously computed solution
    Then all solution words match the pinned GGML CPU evaluator

  @VT-135 @build
  Scenario: Prism routes nonlinear four-bit row writes and copies through Verdant encoders against GGML CPU
    Given floating-point source rows and nonlinear four-bit destination storage
    When a GGML graph copies the tensor or updates indexed rows
    Then the resulting tensor matches the imported CPU backend

  @VT-136 @build
  Scenario: Custom GGML callbacks retain their tensor descriptors, user data, task indices and synchronous graph completion
    Given an upstream custom graph with in-place or distinct output storage
    When the host callback runs at the graph task width
    Then all tasks finish with the same inputs and outputs as GGML CPU

  @VT-137 @build
  Scenario: Prism schedules GELU_ERF and GEGLU_ERF using the Verdant binary32 Gaussian error function
    Given the imported unary and combined or split gated Gaussian error graphs
    When vTensor applies the activation and any gate before the destination conversion
    Then the results agree with the GGML CPU evaluator

  @VT-139 @build
  Scenario: Prism accumulates one- and two-dimensional image-column gradients in GGML kernel order using Verdant binary32 addition
    Given an image-column gradient with a padded, strided or dilated kernel
    When vTensor computes the input gradient
    Then each ordered binary32 sum agrees with the GGML CPU backward graph

  @VT-140 @build
  Scenario: Prism scatters average and first-maximum pooling gradients through Verdant in upstream GGML window order
    Given an incoming pooling gradient and the original input across overlapping windows
    When vTensor accumulates each average or winning gradient
    Then every destination word agrees with the GGML CPU backend

  @VT-141 @build
  Scenario: Prism plans timestep embedding frequencies and destination lanes and Verdant evaluates the circular functions against the imported GGML CPU
    Given GGML timesteps with even and odd embedding dimensions
    When the provider emits cosine and sine halves for each timestep
    Then the lanes and odd trailing zero agree with the pinned CPU backend
