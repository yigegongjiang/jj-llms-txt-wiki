> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eskalasi keputusan sulit dengan alat advisor

> Pasangkan model utama Anda dengan model advisor yang lebih kuat yang dikonsultasikan Claude pada momen-momen kunci selama tugas.

<Note>
  Alat advisor bersifat eksperimental dan memerlukan API Anthropic. Alat ini tidak tersedia di Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, atau Microsoft Foundry. Perilaku, harga, dan ketersediaan dapat berubah.
</Note>

Alat advisor memungkinkan Claude untuk berkonsultasi dengan model kedua yang biasanya lebih kuat pada momen-momen kunci selama tugas, seperti sebelum berkomitmen pada pendekatan, ketika terjebak pada kesalahan berulang, atau sebelum menyatakan tugas selesai. Advisor menerima percakapan lengkap, termasuk setiap pemanggilan alat dan hasilnya, dan mengembalikan panduan yang diterapkan Claude sebelum melanjutkan.

Advisor berjalan di sisi server pada infrastruktur Anthropic sebagai [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), tersedia untuk akun berlangganan dan berbasis API. Anda memilih model mana yang bertindak sebagai advisor, dan Claude memutuskan kapan memanggilnya.

Halaman ini mencakup cara mengaktifkan advisor, pasangan model mana yang diterima, apa yang ditampilkan Claude selama konsultasi, dan bagaimana penggunaan advisor ditagih.

<h2 id="when-to-use-the-advisor">
  Kapan menggunakan advisor
</h2>

Advisor cocok untuk tugas multi-langkah yang panjang di mana sebagian besar giliran bersifat rutin tetapi kualitas rencana menentukan hasilnya. Contohnya termasuk refaktor besar, sesi debugging di mana kesalahan terus berulang, dan tugas yang ingin Anda periksa secara independen sebelum Claude menyatakan selesai.

Ini menambah nilai lebih sedikit pada tugas pendek di mana ada sedikit untuk direncanakan, atau pada pekerjaan di mana setiap giliran memerlukan model terkuat. Untuk itu, [ubah model utama](/docs/id/model-config#setting-your-model) sebagai gantinya, atau lihat [bagaimana advisor dibandingkan dengan opusplan dan subagents](#compare-with-related-features) untuk cara lain mendapatkan pendapat kedua.

<h2 id="enable-the-advisor">
  Aktifkan advisor
</h2>

Anda dapat mengatur model advisor dengan tiga cara:

* **Perintah `/advisor`**: atur atau ubah advisor di tengah sesi dan simpan sebagai default Anda
* **Pengaturan `advisorModel`**: konfigurasi default persisten di [file pengaturan](/docs/id/settings) Anda
* **Bendera `--advisor`**: atur advisor untuk sesi tunggal saat peluncuran

Jika salah satu dari ini mengatur model advisor, advisor diaktifkan untuk sesi yang model utamanya [mendukungnya](#choose-an-advisor-model). Untuk berhenti menggunakannya, lihat [Matikan advisor](#turn-the-advisor-off).

<Note>
  Untuk menggunakan Fable 5 sebagai advisor, Anda memerlukan Claude Code v2.1.170 atau lebih baru dan [akses Fable 5](/docs/id/model-config#work-with-fable-5) untuk organisasi Anda.
</Note>

<h3 id="use-the-/advisor-command">
  Gunakan perintah `/advisor`
</h3>

Jalankan `/advisor` tanpa argumen untuk membuka pemilih yang mencantumkan model advisor yang tersedia, atau teruskan model secara langsung:

```
/advisor opus
```

Pilihan Anda disimpan ke `advisorModel` dalam pengaturan pengguna Anda dan bertahan di seluruh sesi. Jika [`availableModels`](/docs/id/model-config#restrict-model-selection) allowlist organisasi Anda mengecualikan model advisor yang disimpan, advisor tidak dipanggil sampai Anda memilih model yang diizinkan dengan `/advisor`. Jika model utama Anda saat ini tidak mendukung advisor, pilihan masih disimpan dan diaktifkan ketika Anda beralih ke [model utama yang kompatibel](#choose-an-advisor-model) dengan [`/model`](/docs/id/model-config#setting-your-model).

<h3 id="set-advisormodel-in-settings">
  Atur `advisorModel` dalam pengaturan
</h3>

Untuk mengonfigurasi advisor sebagai default tanpa membuka sesi, aturnya di file pengaturan Anda:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Gunakan bendera `--advisor`
</h3>

Untuk mengatur advisor untuk sesi tunggal tanpa mengubah pengaturan yang disimpan, luncurkan dengan bendera:

```bash theme={null}
claude --advisor opus
```

Bendera ini mengambil alih pengaturan `advisorModel` untuk sesi itu. Bendera keluar dengan kesalahan jika model utama sesi tidak mendukung advisor, atau jika model advisor yang diminta dikecualikan oleh allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda.

<h2 id="choose-an-advisor-model">
  Pilih model advisor
</h2>

Advisor harus setidaknya sama mampu dengan model utama. Advisor yang diterima untuk setiap model utama adalah:

| Model utama              | Advisor yang diterima     | Catatan                                                                                                                                                              |
| ------------------------ | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Haiku 4.5                | Fable, Opus, Sonnet       | Haiku dapat memanggil advisor tetapi tidak dapat bertindak sebagai advisor                                                                                           |
| Sonnet 4.6               | Fable, Opus, Sonnet       |                                                                                                                                                                      |
| Sonnet 5                 | Fable, Opus, Sonnet 5     | Advisor Sonnet 4.6 ditolak                                                                                                                                           |
| Opus 4.6                 | Fable, Opus, Sonnet 5     | Sonnet 5 dan Opus 4.6 diperingkat sebagai sama mampu, jadi Opus 4.6 utama menerima advisor Sonnet 5                                                                  |
| Opus 4.7 atau lebih baru | Fable, Opus 4.7, Opus 4.8 | Opus 4.7 dan Opus 4.8 diperingkat sebagai sama mampu, jadi keduanya menerima yang lain sebagai advisor. Opus 4.7 utama dengan advisor Opus 4.6 atau Sonnet 5 ditolak |
| Fable 5 (v2.1.170+)      | Fable                     | Advisor Opus atau Sonnet ditolak                                                                                                                                     |

Fable 5 memerlukan Claude Code v2.1.170 atau lebih baru dan akses Fable 5, baik bertindak sebagai model utama atau advisor.

Atur advisor sebagai `opus`, `sonnet`, atau `fable`. Alias ini diselesaikan ke versi terbaru dari setiap model. Anda juga dapat melewatkan ID model lengkap seperti `claude-opus-4-8`.

Subagent mewarisi advisor yang dikonfigurasi dan menerapkan pemeriksaan pasangan yang sama terhadap model mereka sendiri.

Claude Code memvalidasi pasangan sebelum mengirim permintaan:

* Jika advisor kurang mampu daripada model utama, advisor tidak dilampirkan ke permintaan model utama. Output perintah `/advisor` dan notifikasi menunjukkan hal ini. Subagent yang model mereka sendiri memenuhi pasangan mungkin masih menggunakan advisor.
* Jika model utama atau advisor adalah model yang Claude Code tidak kenali, advisor tidak dilampirkan.

<h3 id="common-model-pairings">
  Pasangan model umum
</h3>

Pasangan apa pun yang diterima berfungsi. Kombinasi ini menyeimbangkan biaya terhadap kemampuan dengan cara yang berbeda:

| Pasangan                      | Kapan menggunakan                                                                                                                                                                                             |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sonnet utama + advisor Opus   | Sonnet menangani pekerjaan rutin dan eskalasi perencanaan, kegagalan ambigu, dan pemeriksaan penyelesaian ke Opus                                                                                             |
| Sonnet utama + advisor Fable  | Panduan Fable 5 pada titik keputusan tanpa menjalankan Fable 5 di seluruh. Memerlukan v2.1.170 atau lebih baru dan akses Fable 5                                                                              |
| Haiku utama + advisor Opus    | Model utama dengan biaya terendah dengan perencanaan yang kuat. Harapkan biaya lebih tinggi daripada Haiku saja tetapi lebih rendah daripada beralih model utama ke Sonnet atau Opus                          |
| Opus utama + advisor Opus     | Opus kedua meninjau yang pertama. Berguna untuk tugas berisiko tinggi di mana pemeriksaan independen lebih penting daripada biaya                                                                             |
| Fable utama + advisor Fable   | Pasangan kemampuan tertinggi ketika Fable 5 tersedia (v2.1.170+). Fable adalah tingkat yang lebih tinggi daripada Opus dan Sonnet, jadi ini adalah satu-satunya advisor yang diterima untuk model utama Fable |
| Sonnet utama + advisor Sonnet | Pendapat kedua dengan biaya lebih rendah untuk menangkap pengawasan rutin                                                                                                                                     |

<h2 id="when-claude-consults-the-advisor">
  Kapan Claude berkonsultasi dengan advisor
</h2>

Claude memutuskan kapan memanggil advisor. Cenderung berkonsultasi sebelum berkomitmen pada pendekatan, ketika kesalahan terus berulang, dan sebelum menyatakan tugas selesai, tetapi waktu didorong oleh model daripada berbasis aturan.

Anda dapat meminta konsultasi dalam prompt Anda dengan cara yang sama seperti Anda meminta alat apa pun, misalnya `consult the advisor before you continue`. Tidak ada pengaturan untuk membatasi atau memaksa panggilan advisor; jika Anda ingin Claude berkonsultasi lebih atau kurang sering selama tugas, katakan dalam instruksi Anda.

<h2 id="what-you-see-during-a-session">
  Apa yang Anda lihat selama sesi
</h2>

Ketika Claude memanggil advisor, transkrip menampilkan baris `Advising` dengan nama model advisor saat panggilan sedang berlangsung. Ketika hasilnya kembali, baris mengkonfirmasi bahwa advisor telah meninjau percakapan. Tekan `Ctrl+O` untuk memperluas dan membaca panduan lengkap advisor.

Claude umumnya mengikuti panduan advisor, tetapi beradaptasi ketika bukti miliknya bertentangan dengan klaim spesifik: jika langkah yang direkomendasikan gagal saat dicoba, atau isi file bertentangan dengan saran, Claude menampilkan konflik daripada mengikuti panduan secara tidak terbatas.

Advisor selalu menerima percakapan lengkap, dan Claude mengontrol waktu. Untuk kontrol lebih atau konfigurasi berbeda, lihat [bagaimana advisor dibandingkan dengan subagents dan opusplan](#compare-with-related-features).

<h2 id="cost">
  Biaya
</h2>

Setiap panggilan advisor mengirim percakapan ke model advisor, jadi mengonsumsi token pada tarif model advisor sebagai tambahan dari penggunaan model utama Anda. Dengan penagihan API, token advisor ditagih pada tarif input dan output model advisor. Pada paket berlangganan, penggunaan advisor dihitung terhadap batas penggunaan paket Anda.

Claude memanggil advisor pada titik keputusan daripada pada setiap giliran, jadi memasangkan model utama yang lebih cepat dengan advisor yang lebih kuat biasanya biaya lebih rendah daripada menjalankan model yang lebih kuat di seluruh. Penggunaan advisor dihitung terhadap total sesi yang ditampilkan oleh [`/usage`](/docs/id/costs#track-your-costs).

Untuk bagaimana token advisor dilaporkan dalam respons API, lihat [Usage and billing](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) dalam dokumentasi Claude API.

<h2 id="impact-on-prompt-caching">
  Dampak pada prompt caching
</h2>

Mengaktifkan atau menonaktifkan advisor di tengah sesi tidak membatalkan [prompt cache](/docs/id/prompt-caching) model utama Anda. Tidak seperti [mengubah model atau tingkat upaya](/docs/id/prompt-caching#actions-that-invalidate-the-cache), mengalihkan `/advisor` menjaga awalan yang di-cache tetap utuh, dan panduan yang dikembalikan advisor di-cache sebagai bagian dari transkrip pada giliran berikutnya.

Pembacaan percakapan model advisor sendiri tidak di-cache. Setiap panggilan advisor memproses transkrip lengkap baru, tanpa penggunaan kembali di antara panggilan.

<h2 id="requirements">
  Persyaratan
</h2>

Alat advisor memerlukan semua hal berikut:

* **Hanya API Anthropic**: advisor adalah alat yang dieksekusi server. Alat ini tidak tersedia di Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, atau Microsoft Foundry. Melalui [LLM gateway](/docs/id/llm-gateway) yang dikonfigurasi dengan `ANTHROPIC_BASE_URL`, ketersediaan tergantung pada apakah gateway meneruskan permintaan utuh ke API Anthropic.
* **Model utama yang didukung**: Opus 4.6 atau lebih baru, Sonnet 4.6 atau lebih baru, atau Haiku 4.5. Fable 5 juga memenuhi syarat di Claude Code v2.1.170 atau lebih baru.

<h2 id="turn-the-advisor-off">
  Matikan advisor
</h2>

Untuk berhenti menggunakan advisor dan menghapus `advisorModel` yang disimpan, jalankan `/advisor off` atau pilih **No advisor** di pemilih `/advisor`:

```
/advisor off
```

Untuk menonaktifkan alat advisor sepenuhnya, atur `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. Perintah `/advisor` menjadi tidak tersedia dan `advisorModel` yang dikonfigurasi apa pun diabaikan. Bendera `--advisor` diterima tetapi tidak memiliki efek; skrip yang ada yang meneruskannya terus berfungsi tanpa kesalahan. Lihat [Environment variables](/docs/id/env-vars).

<h2 id="compare-with-related-features">
  Bandingkan dengan fitur terkait
</h2>

Advisor adalah salah satu dari beberapa cara untuk menggabungkan kekuatan model. Pilih berdasarkan kapan Anda ingin model kedua terlibat.

| Pendekatan                                                       | Kapan model yang lebih kuat berjalan                                                                                                                | Bagaimana dimulai                                   |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| Alat advisor                                                     | Pada titik keputusan di tengah tugas                                                                                                                | Claude memanggilnya ketika memerlukan panduan       |
| [`opusplan`](/docs/id/model-config#opusplan-model-setting)            | Selama mode rencana ketika [diizinkan oleh `availableModels`](/docs/id/model-config#restrict-model-selection), kemudian beralih ke Sonnet untuk eksekusi | Anda memasuki mode rencana                          |
| [Subagents](/docs/id/sub-agents#choose-a-model) dengan `model` diatur | Untuk seluruh subtask yang didelegasikan                                                                                                            | Claude mendelegasikan, atau Anda memanggil subagent |
| [`/model`](/docs/id/model-config#setting-your-model)                  | Untuk semua giliran berikutnya                                                                                                                      | Anda beralih model                                  |

<h2 id="see-also">
  Lihat juga
</h2>

* [Model configuration](/docs/id/model-config): ubah model, atur tingkat upaya, dan gunakan `opusplan`
* [Manage costs effectively](/docs/id/costs): lacak penggunaan token di seluruh model
* [Advisor tool in the Claude API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): pahami alat server yang mendasar, atau gunakan langsung dari Messages API
* [The advisor strategy](https://claude.com/blog/the-advisor-strategy): mengapa memasangkan model utama yang cepat dengan advisor yang lebih kuat berfungsi
