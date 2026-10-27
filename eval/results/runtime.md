# Runtime

CPU only (Intel64 Family 6 Model 170 Stepping 4, GenuineIntel), best of 3 runs for documents. Each language's evaluation sentences are joined into one document of the given word count; the sentence column runs them one by one.

| config | lang | words | document_ms_per_1k_words | sentences_ms_per_1k_words |
|---|---|---|---|---|
| rules | en | 632 | 1.50 | 2.40 |
| rules | fr | 694 | 1.50 | 2.30 |
| rules | de | 655 | 2.10 | 2.60 |
| rules | lb | 656 | 1.50 | 2.30 |
| rules_ner_post | en | 632 | 13757.00 | 52653.50 |
| rules_ner_post | fr | 694 | 15787.30 | 45143.80 |
| rules_ner_post | de | 655 | 14495.40 | 46557.30 |
| rules_ner_post | lb | 656 | 18234.90 | 49234.20 |
