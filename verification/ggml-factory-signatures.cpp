// Metadata reader for the pinned upstream evaluator. No provider or tensor data is loaded.
#define main original_backend_ops_main
#include "tests/test-backend-ops.cpp"
#undef main
#include <map>
#include <iostream>
#include <algorithm>
#include <vector>
#include <sstream>

namespace {
const ggml_tensor * storage(const ggml_tensor * t) { return t->view_src ? t->view_src : t; }

void descriptor(std::ostream & out, const ggml_tensor * t, bool layouts) {
    out << int(t->type) << '/' << t->flags << '/'
        << (ggml_is_contiguous(t) ? 'c' : 's')
        << (ggml_is_permuted(t) ? 'p' : '-')
        << (ggml_is_transposed(t) ? 't' : '-')
        << (!t->view_src ? '-' : t->view_offs == 0 ? 'v' : 'o');
    if (layouts) {
        for (auto extent : t->ne) out << '/' << extent;
        for (auto stride : t->nb) out << '/' << stride;
        out << '/' << t->view_offs;
    }
}

std::string signature(const ggml_tensor * t, bool layouts) {
    std::ostringstream out;
    out << int(t->op) << '\t';
    if (t->op == GGML_OP_UNARY) out << int(ggml_get_unary_op(t));
    else if (t->op == GGML_OP_GLU) out << int(ggml_get_glu_op(t));
    else out << '-';
    if (layouts) for (auto word : t->op_params) out << '/' << uint32_t(word);
    out << '\t';
    descriptor(out, t, layouts);
    for (int i = 0; i < GGML_MAX_SRC; ++i) {
        auto * source = t->src[i];
        if (!source) continue;
        out << '\t' << i << '=';
        descriptor(out, source, layouts);
        if (storage(t) == storage(source)) out << (t->view_offs == source->view_offs ? ":same" : ":offset");
    }
    return out.str();
}
} // namespace

int main(int argc, char **argv) {
    const bool layouts = argc == 2 && std::string(argv[1]) == "--layouts";
    if (argc != 1 && !layouts) return 2;
    const auto tests = make_test_cases_eval();
    std::map<std::string, size_t> dictionary;
    for (size_t i = 0; i < tests.size(); ++i) {
        auto & test = *tests[i];
        ggml_init_params params{ggml_tensor_overhead()*128 + ggml_graph_overhead(), nullptr, true};
        ggml_context_ptr ctx(ggml_init(params)), weights(test.use_weight_context() ? ggml_init(params) : nullptr);
        test.mode = MODE_TEST;
        test.gf = ggml_new_graph(ctx.get());
        test.add_sentinel(ctx.get());
        if (weights) test.add_sentinel(weights.get());
        auto * out = test.build_graph(ctx.get(), weights.get());
        ggml_build_forward_expand(test.gf, out);
        std::vector<size_t> ids;
        auto record = [&](const ggml_tensor * t) {
            auto entry = dictionary.emplace(signature(t, layouts), dictionary.size());
            if (entry.second) std::cout << "S\t" << entry.first->second << '\t' << entry.first->first << '\n';
            ids.push_back(entry.first->second);
        };
        record(out);
        for (int n = 0; n < ggml_graph_n_nodes(test.gf); ++n) {
            auto * node = ggml_graph_node(test.gf, n);
            record(node);
            for (auto * source : node->src) if (source) record(source);
        }
        if (!layouts) {
            std::sort(ids.begin(), ids.end());
            ids.erase(std::unique(ids.begin(), ids.end()), ids.end());
        }
        std::cout << "C\t" << i << '\t' << test.op_desc(out) << '\t' << test.vars() << '\t';
        for (auto it = ids.begin(); it != ids.end(); ++it) std::cout << (it == ids.begin() ? "" : ",") << *it;
        std::cout << '\n';
    }
    return std::cout ? 0 : 1;
}
