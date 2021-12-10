#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

const int XDP_ACTION_MAX = (XDP_TX + 1);

struct datarec {
	__u64 rx_packets;
};

// stats keyed by XDP Action
struct {
	__uint(type, BPF_MAP_TYPE_ARRAY);
	__type(key, __u32);
	__type(value, struct datarec);
	__uint(max_entries, XDP_ACTION_MAX);
} xdp_stats_map SEC(".maps");

// tracks number of times called
struct {
	__uint(type, BPF_MAP_TYPE_ARRAY);
	__type(key, __u32);
	__type(value, __u64);
	__uint(max_entries, 1);
} prog_stats_map SEC(".maps");


SEC("xdp/stats")
int  xdp_stats(struct xdp_md *ctx)
{
    __u64 *stats;
	struct datarec *rec;
	__u32 key = XDP_PASS;
	__u32 k1 = 0;

    stats = bpf_map_lookup_elem(&prog_stats_map, &k1);
    if (!stats)
        return XDP_ABORTED;
    __sync_fetch_and_add(stats, 1);

	rec = bpf_map_lookup_elem(&xdp_stats_map, &key);
	if (!rec)
		return XDP_ABORTED;
	__sync_fetch_and_add(&rec->rx_packets, 1);

	return XDP_PASS;
}

char _license[] SEC("license") = "GPL";