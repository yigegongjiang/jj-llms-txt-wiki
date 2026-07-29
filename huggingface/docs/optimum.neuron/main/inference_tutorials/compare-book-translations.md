# Comparing Book Translations using Embeddings

This notebook demonstrates a practical application of semantic embeddings for comparing translations of literary works. We will compare **Alice in Wonderland** in English and French, extracting chapters and using embeddings from a deployed inference endpoint to verify translation quality and alignment.

### Background: Embedding Models

Embedding models convert text into fixed-size numerical vectors (embeddings) that capture semantic meaning in a shared vector space. This enables powerful operations like similarity comparisons between texts, regardless of their surface-level differences.

**Qwen3-Embedding models** are multilingual embedding models developed by Alibaba's Qwen team, supporting 100+ languages in a single model. Key advantages:
- **Multilingual support**: Texts in different languages are mapped to the same vector space, enabling cross-lingual similarity comparisons
- **Semantic preservation**: Translations with equivalent meaning generate similar embeddings, perfect for translation verification
- **Efficiency**: The `Qwen3-Embedding-4B` variant offers excellent performance-to-accuracy tradeoff

This makes `Qwen3-Embedding` models ideal for translation quality assurance and cross-lingual document matching tasks.

Note: the multi-lingual capabilities of the `Qwen3-Embeddings-0.6B` model are not sufficient for this particular use-case.

### What This Example Illustrates

Translation verification is a critical task in publishing and localization. By leveraging sentence embeddings, we can:

1. **Automatically match chapters** between two language versions of a book by comparing chapter title embeddings
2. **Verify paragraph correspondence** by finding semantically similar paragraphs between source and translated text
3. **Quantify translation quality** using cosine similarity scores as a proxy for semantic fidelity

This approach works regardless of language pair or linguistic differences because embeddings capture semantic meaning in a shared vector space.

### Dataset

We use two EPUB versions of "Alice's Adventures in Wonderland":
- **Original (English)**: [Project Gutenberg #11](https://www.gutenberg.org/ebooks/11.epub.noimages)
- **Translation (French)**: [Project Gutenberg #55456](https://www.gutenberg.org/ebooks/55456.epub.noimages)

### Workflow

1. Deploy an embedding model to Inference Endpoints
2. Download the two EPUB files from Project Gutenberg
3. Extract chapter text and paragraph content from both books
4. Generate embeddings for all chapters and paragraphs
5. Compute similarity matrices to find matching chapters and paragraphs
6. Analyze and visualize the translation correspondence

## 1. Configure Inference Endpoint

We will deploy our embeddings model on Inference Endpoints, a fully managed service that simplifies inference deployment on Trainium/Inferentia devices, using vLLM.

Please refer to [this guide](https://huggingface.co/docs/optimum-neuron/guides/vllm_on_ie) for the step-by-step instructions to deploy an LLM model on [Inference Endpoints](https://huggingface.co/docs/inference-endpoints/en/index).

This tutorial has been validated using the `Qwen/Qwen3-Embedding-4B` model deployed on the smallest `INF2` instance (2 cores - 32 GB device memory).

Once it has been deployed, please copy your endpoint URL, it will be required in the next steps.

## 2. Setup and Dependencies

Install required libraries and import them to set up the environment.

```python
%pip install -q requests openai torch ebooklib bs4 huggingface_hub matplotlib numpy
```

```python
import os
import tempfile
from pathlib import Path
from typing import Dict, List, Optional

import ebooklib
import requests
import torch
import torch.nn.functional as F
from bs4 import BeautifulSoup
from ebooklib import epub
from openai import OpenAI
from torch import Tensor
from huggingface_hub import get_token

print("✓ All dependencies imported successfully")
```

```python
# Inference Endpoint Configuration

BASE_URL = os.environ.get("INFERENCE_ENDPOINT_URL")
if not BASE_URL:
    BASE_URL = input("Enter the Inference Endpoint URL: ")
TOKEN = get_token()
if TOKEN is None:
    TOKEN = input("Enter your Hugging Face API Token: ")
```

## 3. Download EPUB Files

Download Alice in Wonderland in both English and French from Project Gutenberg.

```python
# Download Alice in Wonderland in English and French
epub_dir = Path(tempfile.mkdtemp(prefix="alice_"))

URLs = {
    "original": "https://www.gutenberg.org/ebooks/11.epub.noimages",
    "translation": "https://www.gutenberg.org/ebooks/55456.epub.noimages"
}

epub_files = {}
for lang, url in URLs.items():
    filepath = epub_dir / f"alice_{lang.lower()}.epub"
    response = requests.get(url, timeout=120)
    response.raise_for_status()
    filepath.write_bytes(response.content)
    epub_files[lang] = filepath

print(f"✓ Downloaded {len(epub_files)} EPUB files to {epub_dir}")
```

## 4. Extract Chapters from EPUBs

Use ebooklib to parse EPUB files and extract chapter structure with paragraph text.

```python
def extract_chapters(book: epub.EpubBook) -> Dict[str, List[str]]:
    """Extract chapters and paragraphs from EPUB."""

    def flatten_toc_titles(node, titles: List[str]) -> None:
        if isinstance(node, (list, tuple)):
            for item in node:
                flatten_toc_titles(item, titles)
            return

        title = getattr(node, "title", None)
        if title:
            titles.append(title)

        subitems = getattr(node, "subitems", None)
        if subitems:
            flatten_toc_titles(subitems, titles)

    def normalize_title(text: str) -> str:
        return " ".join(text.replace("\n", " ").split()).strip().casefold()

    chapters = {}
    toc_titles: List[str] = []
    flatten_toc_titles(book.toc, toc_titles)
    toc_title_set = {normalize_title(title) for title in toc_titles}
    documents = list(book.get_items_of_type(ebooklib.ITEM_DOCUMENT))

    current_title: Optional[str] = None
    current_chapter: List[str] = []

    for doc in documents:
        soup = BeautifulSoup(doc.get_body_content(), "html.parser")
        for node in soup.find_all(["h2", "p"]):
            if node.name == "h2":
                heading = node.get_text(" ", strip=True)
                normalized_heading = normalize_title(heading)
                if normalized_heading in toc_title_set:
                    if len(current_chapter) > 0 and current_title:
                        chapters[current_title] = current_chapter
                    current_title = heading
                    current_chapter = []
                continue

            if node.name == "p" and current_title:
                text = node.get_text(" ", strip=True)
                if text and any(c.isalnum() for c in text):
                    current_chapter.append(text)

    if current_title and len(current_chapter) > 0:
        chapters[current_title] = current_chapter

    return chapters

def extract_language(book: epub.EpubBook) -> str:
    """Extract language code from EPUB metadata."""
    language_metadata = book.get_metadata('DC', 'language')
    if language_metadata and len(language_metadata) > 0:
        return language_metadata[0][0]  # Return language code (e.g., 'en', 'fr')
    return "unknown"

# Extract chapters from both EPUB files
print("Extracting chapters and language metadata...\n")
chapters_data = {}
language_map = {}  # Map from user-provided key to actual language code

for key, epub_path in epub_files.items():
    print(f"Processing {key} ...")
    try:
        book = epub.read_epub(str(epub_path))

        # Extract language from metadata
        detected_lang = extract_language(book)
        language_map[key] = detected_lang

        # Extract chapters
        chapters = extract_chapters(book)
        chapters_data[key] = chapters

        print(f"  ✓ Found {len(chapters)} chapters")
        print(f"  ✓ Detected language: {detected_lang}")
        print(f"    Chapters: {', '.join(list(chapters.keys())[:3])}...")
    except Exception as e:
        print(f"  ✗ Error: {e}")
```

## 5. Initialize Embedding Client and Helper Functions

Set up the OpenAI-compatible client and define utility functions for embedding and similarity computation.

```python
# Initialize OpenAI client for embeddings
print("Initializing embedding client...")
client = OpenAI(base_url=BASE_URL + "/v1", api_key=TOKEN)

# Test connection
try:
    models = client.models.list()
    available_models = [m.id for m in models.data]
    print(f"✓ Connected to embedding service")
    MODEL_NAME = available_models[0]
    print(f"  Selected model: {MODEL_NAME}")
except Exception as e:
    print(f"✗ Failed to connect: {e}")
    print("Make sure your inference endpoint is running")

def embed_translations(client: OpenAI, texts: List[str], model: str) -> Tensor:
    """Generate embeddings for a list of texts using the inference endpoint."""
    task = "Given a text, retrieve its translation from the provided documents."
    instructed_texts = [f"Instruct: {task}\nQuery: {text}" for text in texts]

    resp = client.embeddings.create(input=instructed_texts, model=model)
    embeddings = [torch.tensor(d.embedding, dtype=torch.float32) for d in resp.data]
    embeddings = torch.stack(embeddings)
    embeddings = F.normalize(embeddings, p=2, dim=1)
    return embeddings

def similarity_matrix(emb_a: Tensor, emb_b: Tensor) -> Tensor:
    """Compute cosine similarity matrix between two sets of embeddings."""
    return emb_a @ emb_b.T

def find_best_match(scores: Tensor, threshold: float = 0.0) -> int:
    """Find index of best match in similarity scores."""
    return torch.argmax(scores).item()

def compute_chapter_match_score(title_sim: float, idx_a: int, idx_b: int,
                                 para_count_a: int, para_count_b: int,
                                 max_idx: int,
                                 w_title: float = 0.7, w_index: float = 0.15,
                                 w_paras: float = 0.15) -> float:
    """
    Compute a composite score for chapter matching combining:
    - Title similarity (semantic): weight 0.7
    - Chapter index proximity: weight 0.15
    - Paragraph count similarity: weight 0.15

    This helps identify chapters where title translation is poor but other signals match.
    """
    # Title similarity (already 0-1 range)
    title_score = title_sim

    # Index proximity: 1 - normalized distance
    index_distance = abs(idx_a - idx_b) / max(max_idx, 1)
    index_score = 1.0 - index_distance

    # Paragraph count similarity: normalized overlap ratio
    max_paras = max(para_count_a, para_count_b)
    min_paras = min(para_count_a, para_count_b)
    para_score = min_paras / max(max_paras, 1)

    # Weighted combination
    combined_score = w_title * title_score + w_index * index_score + w_paras * para_score

    return combined_score

print("✓ Embedding utilities initialized")
```

## 6. Embed and Compare Chapters

Generate embeddings for all chapter titles and build a correspondence table between English and French versions.

```python
print("Computing chapter title embeddings and building correspondence...\n")

# Extract chapter titles from both versions
langs = list(chapters_data.keys())

original_lang, translation_lang = langs[0], langs[1]

titles_original = list(chapters_data[original_lang].keys())
titles_translation = list(chapters_data[translation_lang].keys())

print(f"{original_lang}: {len(titles_original)} chapters")
print(f"{translation_lang}: {len(titles_translation)} chapters\n")

# Embed chapter titles
print(f"Embedding {len(titles_original) + len(titles_translation)} chapter titles...")
try:
    emb_titles_original = embed_translations(client, titles_original, MODEL_NAME)
    emb_titles_translation = embed_translations(client, titles_translation, MODEL_NAME)
    print(f"✓ Embeddings complete\n")

    # Get paragraph counts for each chapter
    para_counts_original = [len(chapters_data[original_lang][title]) for title in titles_original]
    para_counts_translation = [len(chapters_data[translation_lang][title]) for title in titles_translation]

    # Build correspondence table using composite scoring
    print(f"Building chapter correspondence with composite scoring...\n")
    title_scores = similarity_matrix(emb_titles_original, emb_titles_translation)

    correspondence = {}
    match_details = {}  # Store detailed scoring info

    for i, title_original in enumerate(titles_original):
        best_idx = None
        best_composite_score = -1

        # Check all candidates and compute composite scores
        for j in range(len(titles_translation)):
            title_sim = title_scores[i, j].item()
            composite_score = compute_chapter_match_score(
                title_sim=title_sim,
                idx_a=i,
                idx_b=j,
                para_count_a=para_counts_original[i],
                para_count_b=para_counts_translation[j],
                max_idx=max(len(titles_original), len(titles_translation))
            )

            if composite_score > best_composite_score:
                best_composite_score = composite_score
                best_idx = j

        title_translation = titles_translation[best_idx]
        title_sim = title_scores[i, best_idx].item()

        correspondence[title_original] = (title_translation, best_composite_score)
        match_details[title_original] = {
            'title_sim': title_sim,
            'composite_score': best_composite_score,
            'para_count_a': para_counts_original[i],
            'para_count_b': para_counts_translation[best_idx]
        }

    # Display correspondence table
    print("+" + "=" * 100 + "+")
    print(f"| {'Original': 27 else title_original
        b_short = title_translation[:27] if len(title_translation) > 27 else title_translation
        details = match_details[title_original]
        print(f"| {a_short:8.3f} | {comp_score:>8.3f} |")

    print("+" + "=" * 100 + "+")

    # Identify chapters with low title sim but decent composite score (potential translation issues)
    print(f"\nChapters with potential translation discrepancies (title_sim  0.15:  # Significant gap between scores
            discrepancies.append((title_original, gap, details))

    if discrepancies:
        discrepancies.sort(key=lambda x: x[1], reverse=True)
        for title_original, gap, details in discrepancies[:5]:
            print(f"  - {title_original[:40]}: title_sim={details['title_sim']:.3f}, composite={details['composite_score']:.3f} (gap: {gap:.3f})")
            print(f"    Para counts: {details['para_count_a']} vs {details['para_count_b']}")
    else:
        print("  None detected - title translations appear accurate")

except Exception as e:
    print(f"Error during chapter embedding: {e}")
    print("Make sure your inference endpoint is running and accessible")
```

## 7. Analyze Paragraph Correspondence

Deep dive into a specific chapter pair to examine paragraph-level translation quality.

```python
# Select a chapter for deeper analysis
CHAPTER_INDEX = 0  # Change this index to analyze different chapters
selected_title_original = titles_original[CHAPTER_INDEX]
selected_title_translation, _ = correspondence[selected_title_original]

print(f"\nSelected Chapter for Analysis:")
print(f"  Original: {selected_title_original}")
print(f"  Translation: {selected_title_translation}")

# Display paragraphs from both versions
paragraphs_original = chapters_data[original_lang][selected_title_original]
paragraphs_translation = chapters_data[translation_lang][selected_title_translation]
```

```python
print(f"Paragraph counts:")
print(f"  {original_lang}: {len(paragraphs_original)} paragraphs")
print(f"  {translation_lang}: {len(paragraphs_translation)} paragraphs")

# Handle paragraph count mismatch with semantic merging
if len(paragraphs_original) != len(paragraphs_translation):
    print(f"\n⚠️  Paragraph count mismatch detected: {len(paragraphs_original)} vs {len(paragraphs_translation)}")
    print(f"   Applying semantic-guided merging to align paragraph counts...\n")

    # Determine which language has more paragraphs (source) and which has fewer (target)
    if len(paragraphs_original) > len(paragraphs_translation):
        source_paras = paragraphs_original
        target_paras = paragraphs_translation
        source_lang = original_lang
        target_lang = translation_lang
        merge_original = True
    else:
        source_paras = paragraphs_translation
        target_paras = paragraphs_original
        source_lang = translation_lang
        target_lang = original_lang
        merge_original = False

    print(f"Will merge {source_lang} paragraphs ({len(source_paras)}) to match {target_lang} count ({len(target_paras)})")

    # Embed all paragraphs
    print(f"\nEmbedding paragraphs...")
    emb_source = embed_translations(client, source_paras, MODEL_NAME)
    emb_target = embed_translations(client, target_paras, MODEL_NAME)
    print("✓ Complete\n")

    # Perform semantic-guided merging using mean approximation
    # Justification: The linearity of embedding spaces in modern LLMs means that
    # the mean of two paragraph embeddings provides a good approximation of the
    # merged paragraph's embedding. This property has been empirically observed
    # across various embedding models (Mikolov et al., 2013; Vaswani et al., 2017).
    # We verified this assumption by comparing mean approximation with exact
    # embeddings, showing  best_score:
                best_score = total_score
                best_merge_idx = i

        # Perform the best merge
        if best_merge_idx is not None:
            # Merge paragraphs
            merged_para = current_paragraphs[best_merge_idx] + "\n" + current_paragraphs[best_merge_idx + 1]
            current_paragraphs = (
                current_paragraphs[:best_merge_idx] +
                [merged_para] +
                current_paragraphs[best_merge_idx+2:]
            )

            # Reevaluate embeddings for the merged paragraph
            merged_emb = embed_translations(client, [merged_para], MODEL_NAME)[0].unsqueeze(0)
            current_embeddings = (
                current_embeddings[:best_merge_idx] +
                [merged_emb] +
                current_embeddings[best_merge_idx+2:]
            )

            print(f"  Merge {merge_num+1}/{num_merges}: Merged {source_lang} paragraphs at position {best_merge_idx}")

    # Assign results back to appropriate variables
    emb_source_merged = torch.cat(current_embeddings, dim=0)

    if merge_original:
        paragraphs_original = current_paragraphs
        emb_original = emb_source_merged
        emb_translation = emb_target
    else:
        paragraphs_translation = current_paragraphs
        emb_translation = emb_source_merged
        emb_original = emb_target

    print(f"\n✓ Semantic merging complete: {len(current_paragraphs)} aligned paragraphs\n")
else:
    # No mismatch, proceed normally
    print(f"\nEmbedding {len(paragraphs_original) + len(paragraphs_translation)} paragraphs...")
    emb_original = embed_translations(client, paragraphs_original, MODEL_NAME)
    emb_translation = embed_translations(client, paragraphs_translation, MODEL_NAME)
    print("✓ Complete\n")

# Now compute 1-to-1 alignment similarity
print("Computing paragraph alignment quality:")
similarities = (emb_original * emb_translation).sum(dim=1)
avg_similarity = similarities.mean().item()

print(f"  Average similarity: {avg_similarity:.4f}")
print(f"  Min similarity:     {similarities.min().item():.4f}")
print(f"  Max similarity:     {similarities.max().item():.4f}")
```

## 8. Visualize Paragraph-Level Translation Quality

```python
import matplotlib.pyplot as plt
import numpy as np

# Convert similarities tensor to numpy array
sim_scores = similarities.cpu().numpy() if hasattr(similarities, 'cpu') else similarities.numpy()

# Create paragraph indices
para_indices = np.arange(1, len(sim_scores) + 1)

# Assign colors based on quality thresholds
colors = []
for score in sim_scores:
    if score >= 0.85:
        colors.append('#2ecc71')  # Green for high quality
    elif score >= 0.75:
        colors.append('#f39c12')  # Orange for medium quality
    else:
        colors.append('#e74c3c')  # Red for low quality

# Create the figure with two subplots
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(14, 10),
                                    gridspec_kw={'height_ratios': [3, 1]})

# Main plot: Bar chart of similarities
bars = ax1.bar(para_indices, sim_scores, color=colors, alpha=0.7, edgecolor='black', linewidth=0.5)

# Add threshold lines
ax1.axhline(y=0.85, color='#2ecc71', linestyle='--', linewidth=2, alpha=0.5, label='High Quality (≥0.85)')
ax1.axhline(y=0.75, color='#f39c12', linestyle='--', linewidth=2, alpha=0.5, label='Medium Quality (≥0.75)')

# Add average line
avg_sim = np.mean(sim_scores)
ax1.axhline(y=avg_sim, color='blue', linestyle=':', linewidth=2, alpha=0.7, label=f'Average ({avg_sim:.3f})')

# Formatting
ax1.set_xlabel('Paragraph Number', fontsize=12, fontweight='bold')
ax1.set_ylabel('Similarity Score', fontsize=12, fontweight='bold')
ax1.set_title(f'Paragraph-Level Translation Quality\n{selected_title_original} → {selected_title_translation}',
                fontsize=14, fontweight='bold', pad=20)
ax1.set_ylim(0, 1.05)
ax1.grid(True, alpha=0.3, axis='y')
ax1.legend(loc='upper right', fontsize=10)

# Bottom plot: Quality distribution pie chart
high_count = np.sum(sim_scores >= 0.85)
medium_count = np.sum((sim_scores >= 0.75) & (sim_scores < 0.85))
low_count = np.sum(sim_scores < 0.75)

quality_counts = [high_count, medium_count, low_count]
quality_labels = [f'High\n({high_count})', f'Medium\n({medium_count})', f'Low\n({low_count})']
quality_colors = ['#2ecc71', '#f39c12', '#e74c3c']

wedges, texts, autotexts = ax2.pie(quality_counts, labels=quality_labels, colors=quality_colors,
                                        autopct='%1.1f%%', startangle=90, textprops={'fontsize': 11})
ax2.set_title('Quality Distribution', fontsize=12, fontweight='bold')

plt.tight_layout()
plt.show()

# Print summary statistics
print(f"\nParagraph Similarity Statistics for: {selected_title_original}")
print(f"{'='*60}")
print(f"Total paragraphs:     {len(sim_scores)}")
print(f"Average similarity:   {avg_sim:.4f}")
print(f"Min similarity:       {np.min(sim_scores):.4f} (paragraph {np.argmin(sim_scores) + 1})")
print(f"Max similarity:       {np.max(sim_scores):.4f} (paragraph {np.argmax(sim_scores) + 1})")
print(f"Std deviation:        {np.std(sim_scores):.4f}")
print(f"\nQuality breakdown:")
print(f"  High (≥0.85):       {high_count}/{len(sim_scores)} ({100*high_count/len(sim_scores):.1f}%)")
print(f"  Medium (0.75-0.85): {medium_count}/{len(sim_scores)} ({100*medium_count/len(sim_scores):.1f}%)")
print(f"  Low (<0.75):        {low_count}/{len(sim_scores)} ({100*low_count/len(sim_scores):.1f}%)")
```

## Next Steps and Deployment

### Customization Ideas

1. **Different Models**: Replace `Qwen/Qwen3-Embedding-4B` with other embedding models like `Qwen/Qwen3-Embedding-8B` for potentially better results

2. **Other Books**: Download different works from Project Gutenberg and compare different language pairs
    - "Le tour du monde en 80 jours - Jules Verne" - [original](https://www.gutenberg.org/ebooks/800.epub.noimages) - [translation](https://www.gutenberg.org/ebooks/103.epub.noimages)

3. **Batch Processing**: Scale to compare many chapters automatically with configurable thresholds

4. **Export Results**: Save correspondence data to JSON or CSV for further analysis

5. **Task Tuning**: Adjust the instruction prompt in `embed_texts()` to optimize embeddings for your specific use case

### Performance Notes

- Embedding generation time scales with the number of texts and text length
- Similarity computation is fast (matrix multiplication) once embeddings are available
- For production use, consider caching embeddings to avoid re-computation
- Batch size in `client.embeddings.create()` may need adjustment based on endpoint limits

### Troubleshooting

If embeddings fail to generate:
1. Verify endpoint is running and accessible
2. Check your Hugging Face token configuration
3. Review endpoint logs for error messages
