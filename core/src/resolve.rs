use crate::Span;

/// Keeps the longest span among overlapping ones, then the highest scoring, and returns the
/// survivors ordered by position. Remaining ties are broken by start, type and source so the
/// output never depends on input order.
pub fn resolve(mut spans: Vec<Span>) -> Vec<Span> {
    spans.sort_by(|a, b| {
        b.len()
            .cmp(&a.len())
            .then(b.score.total_cmp(&a.score))
            .then(a.start.cmp(&b.start))
            .then(a.kind.cmp(&b.kind))
            .then(a.source.cmp(&b.source))
    });
    let mut kept: Vec<Span> = Vec::with_capacity(spans.len());
    for s in spans {
        if !kept.iter().any(|k| k.overlaps(&s)) {
            kept.push(s);
        }
    }
    kept.sort_by_key(|s| (s.start, s.end, s.kind, s.source));
    kept
}
