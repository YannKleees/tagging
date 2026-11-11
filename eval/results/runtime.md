# Runtime

CPU only (x86_64), best of 3 runs for documents. Each language's evaluation sentences are joined into one document of the given word count; the sentence column runs them one by one, the batch column in one `detect_batch` call.

| config | lang | words | document_ms_per_1k_words | sentences_ms_per_1k_words | batch_ms_per_1k_words |
|---|---|---|---|---|---|
| rules | en | 632 | 4.40 | 4.20 | 2.90 |
| rules | fr | 694 | 3.90 | 4.90 | 4.20 |
| rules | de | 655 | 4.20 | 4.20 | 3.40 |
| rules | lb | 656 | 4.60 | 5.50 | 4.50 |
| rules_fast | en | 632 | 77.00 | 322.00 | 105.30 |
| rules_fast | fr | 694 | 77.80 | 326.00 | 93.60 |
| rules_fast | de | 655 | 83.20 | 444.60 | 94.90 |
| rules_fast | lb | 656 | 55.00 | 210.80 | 71.70 |
| rules_ner_post | en | 632 | 6905.00 | 17503.80 | 7917.40 |
| rules_ner_post | fr | 694 | 10487.90 | 16203.80 | 8467.30 |
| rules_ner_post | de | 655 | 9232.10 | 16946.70 | 8182.00 |
| rules_ner_post | lb | 656 | 13095.80 | 16988.40 | 8189.90 |
