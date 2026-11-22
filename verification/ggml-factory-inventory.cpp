#define main vtensor_unused_upstream_main
#include "tests/test-backend-ops.cpp"
#undef main
#include <map>
int main() {
  std::map<std::string, size_t> counts;
  size_t ordinal = 0;
  for (auto &test : make_test_cases_eval()) {
    test->mode = MODE_TEST;
    ggml_init_params params = {ggml_tensor_overhead()*128 + ggml_graph_overhead(), nullptr, true};
    ggml_context_ptr ctx(ggml_init(params));
    ggml_context_ptr weights(test->use_weight_context() ? ggml_init(params) : nullptr);
    if (!ctx || (test->use_weight_context() && !weights)) return 2;
    test->gf = ggml_new_graph(ctx.get());
    test->add_sentinel(ctx.get());
    if (weights) test->add_sentinel(weights.get());
    auto *out = test->build_graph(ctx.get(), weights.get());
    auto label = test->op_desc(out);
    ++counts[label];
    printf("CASE\t%zu\t%s\t%s\n", ordinal++, label.c_str(), test->vars().c_str());
  }
  for (const auto &row : counts) fprintf(stderr, "%s\t%zu\n", row.first.c_str(), row.second);
  fprintf(stderr, "TOTAL\t%zu\n", ordinal);
}
