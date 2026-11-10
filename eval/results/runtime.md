# Runtime

CPU only (x86_64), best of 3 runs for documents. Each language's evaluation sentences are joined into one document of the given word count; the sentence column runs them one by one, the batch column in one `detect_batch` call.

| config | lang | words | document_ms_per_1k_words | sentences_ms_per_1k_words | batch_ms_per_1k_words |
|---|---|---|---|---|---|
| rules | en | 632 | 2.70 | 2.40 | 1.60 |
| rules | fr | 694 | 2.50 | 3.00 | 2.60 |
| rules | de | 655 | 2.70 | 2.60 | 2.00 |
| rules | lb | 656 | 2.80 | 12.60 | 2.70 |
| rules_ner_post | en | 632 | 6399.30 | 16791.00 | 7715.00 |
| rules_ner_post | fr | 694 | 8732.50 | 15370.90 | 7241.40 |
| rules_ner_post | de | 655 | 8312.90 | 16822.50 | 7024.20 |
| rules_ner_post | lb | 656 | 11390.80 | 17113.80 | 8113.80 |
