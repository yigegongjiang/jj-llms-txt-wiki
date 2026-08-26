> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi model

> Pelajari tentang konfigurasi model Claude Code, termasuk alias model seperti `opusplan`

<h2 id="available-models">
  Model yang tersedia
</h2>

Untuk pengaturan `model` di Claude Code, Anda dapat mengonfigurasi salah satu dari:

* Sebuah **alias model**
* Sebuah **nama model**
  * Anthropic API: sebuah **[nama model](https://platform.claude.com/docs/id/about-claude/models/overview)** lengkap
  * Amazon Bedrock: ARN profil inferensi
  * Microsoft Foundry: nama deployment
  * Google Cloud's Agent Platform: nama versi

Untuk panduan tentang model mana dan tingkat upaya yang sesuai untuk berbagai jenis pekerjaan, lihat [Memilih model Claude dan tingkat upaya di Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) di blog.

<Note>
  `ANTHROPIC_BASE_URL` mengubah tempat permintaan dikirim, bukan model mana yang menjawabnya. Untuk merutekan Claude melalui gateway LLM, lihat [gateway LLM](/docs/id/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias model
</h3>

Alias model menyediakan cara yang nyaman untuk memilih pengaturan model tanpa perlu mengingat nomor versi yang tepat:

| Alias model      | Perilaku                                                                                                                                                                                                                                                                                                                                                      |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Nilai khusus yang menghapus penggantian model apa pun dan kembali ke model yang direkomendasikan untuk jenis akun Anda, atau ke [model default organisasi](#organization-default-model) ketika admin telah menetapkan satu. Bukan sendiri alias model                                                                                                         |
| **`best`**       | Menggunakan Fable 5 di mana organisasi Anda memiliki akses ke dalamnya, jika tidak maka model Opus terbaru                                                                                                                                                                                                                                                    |
| **`fable`**      | Menggunakan Claude Fable 5 untuk tugas-tugas tersulit dan paling lama Anda                                                                                                                                                                                                                                                                                    |
| **`sonnet`**     | Menggunakan model Sonnet terbaru untuk tugas coding sehari-hari                                                                                                                                                                                                                                                                                               |
| **`opus`**       | Menggunakan model Opus terbaru untuk tugas penalaran kompleks                                                                                                                                                                                                                                                                                                 |
| **`haiku`**      | Menggunakan model Haiku yang cepat dan efisien untuk tugas sederhana                                                                                                                                                                                                                                                                                          |
| **`sonnet[1m]`** | Menggunakan Sonnet dengan [jendela konteks 1 juta token](https://platform.claude.com/docs/id/build-with-claude/context-windows#context-window-sizes-by-model) untuk sesi panjang. Tidak berpengaruh ketika `sonnet` sudah diselesaikan ke Sonnet 5 dengan jendela 1M bawaannya; di belakang [gateway LLM](/docs/id/llm-gateway), memilih jendela 1M untuk Sonnet 5 |
| **`opus[1m]`**   | Menggunakan Opus dengan [jendela konteks 1 juta token](https://platform.claude.com/docs/id/build-with-claude/context-windows#context-window-sizes-by-model) untuk sesi panjang                                                                                                                                                                                |
| **`opusplan`**   | Mode khusus yang menggunakan `opus` selama Plan Mode, kemudian beralih ke `sonnet` untuk eksekusi                                                                                                                                                                                                                                                             |

Versi yang diselesaikan oleh alias `opus` dan `sonnet` tergantung pada penyedia:

| Penyedia                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| Anthropic API                                        | Opus 4.8 | Sonnet 5   |
| [Claude Platform on AWS](/docs/id/claude-platform-on-aws) | Opus 4.8 | Sonnet 4.6 |
| Amazon Bedrock, Google Cloud's Agent Platform        | Opus 4.8 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

Di mana alias diselesaikan ke model yang lebih lama, model yang lebih baru tersedia dengan memilih nama model lengkap secara eksplisit atau mengatur `ANTHROPIC_DEFAULT_OPUS_MODEL` atau `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Sebelum v2.1.207, `opus` diselesaikan ke Opus 4.7 di Claude Platform on AWS dan ke Opus 4.6 di Amazon Bedrock dan Google Cloud's Agent Platform.

Alias menunjuk ke versi yang direkomendasikan untuk penyedia Anda dan diperbarui seiring waktu. Untuk menetapkan versi tertentu, gunakan nama model lengkap, misalnya `claude-opus-4-8`, atau atur variabel lingkungan yang sesuai seperti `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Sonnet 5 memerlukan Claude Code v2.1.197 atau lebih baru. Opus 4.8 memerlukan v2.1.154 atau lebih baru. Jalankan `claude update` untuk meningkatkan.
</Note>

<h3 id="work-with-fable-5">
  Bekerja dengan Fable 5
</h3>

[Claude Fable 5](https://platform.claude.com/docs/id/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) adalah model paling mampu di Claude Code, cocok untuk tugas yang lebih besar dari satu sesi. Ini mempertahankan sesi otonomi yang panjang, menyelidiki sebelum bertindak, dan memverifikasi pekerjaan lebih sering daripada model yang lebih kecil.

Fable 5 bukan model default. Pilih dengan `/model fable`. Permintaan yang ditandai oleh pengklasifikasi keamanannya, paling sering di domain keamanan siber dan biologi, memicu [fallback model otomatis](#automatic-model-fallback).

Untuk mendapatkan hasil maksimal dari Fable 5:

* **Jelaskan hasilnya, bukan langkah-langkahnya**: berikan hasil yang Anda inginkan dan biarkan ia merencanakan jalurnya. Untuk membuatnya terus bekerja sampai hasil itu terpenuhi, [tetapkan tujuan](/docs/id/goal).
* **Berikan masalah yang ambigu**: investigasi akar penyebab, debugging pemadaman, dan keputusan arsitektur adalah tempat investigasi dan verifikasi ekstra memberikan hasil.
* **Lewati pengingat verifikasi**: ia memverifikasi pekerjaan sendiri dengan prompting yang lebih sedikit, jadi pengingat untuk menguji atau memeriksa biasanya tidak perlu.
* **Ukur tugas yang lebih besar**: berikan pekerjaan yang biasanya akan Anda pecah menjadi beberapa bagian. Ia mempertahankan sesi panjang tanpa kehilangan benang merah.

<Note>
  Fable 5 memerlukan Claude Code v2.1.170 atau lebih baru. Versi yang lebih lama tidak menampilkan Fable 5 di pemilih model dan tidak dapat memilihnya. Jalankan `claude update` untuk meningkatkan. Fable 5 tidak tersedia di bawah [zero data retention](/docs/id/zero-data-retention), di mana pemilih `/model` baik menghilangkannya atau menampilkannya dinonaktifkan.
</Note>

<h3 id="setting-your-model">
  Mengatur model Anda
</h3>

Anda dapat mengonfigurasi model Anda dengan beberapa cara, yang tercantum dalam urutan prioritas:

1. **Selama sesi**: gunakan `/model <alias|name>` untuk beralih segera, atau jalankan `/model` tanpa argumen untuk membuka pemilih. Pemilih meminta konfirmasi ketika percakapan memiliki output sebelumnya, karena respons berikutnya membaca ulang riwayat lengkap tanpa konteks cache
2. **Saat startup**: luncurkan dengan `claude --model <alias|name>`
3. **Variabel lingkungan**: atur `ANTHROPIC_MODEL=<alias|name>`
4. **Pengaturan**: konfigurasi secara permanen di file pengaturan Anda menggunakan bidang `model`

Mulai dari v2.1.153, `/model` menyimpan pilihan Anda sebagai default untuk sesi baru dengan menulis bidang `model` di pengaturan pengguna Anda. Di pemilih:

* `Enter`: beralih model dan simpan sebagai default Anda
* `s`: beralih model hanya untuk sesi ini

Mengetik `/model <name>` langsung berperilaku seperti `Enter`. Model yang ditetapkan dengan `/model` dalam [mode non-interaktif](/docs/id/headless), dengan bendera `-p`, berlaku hanya untuk sesi saat ini dan tidak disimpan sebagai default Anda. Pengaturan proyek dan yang dikelola masih memiliki prioritas dan diterapkan kembali pada peluncuran berikutnya. Sebuah [model default organisasi](#organization-default-model) yang admin Anda telah konfigurasi untuk mengganti pilihan pengguna juga diterapkan kembali pada peluncuran berikutnya.

Di v2.1.144 hingga v2.1.152, `/model` hanya berlaku untuk sesi saat ini dan `d` di pemilih menyimpan default.

Bendera `--model` dan variabel lingkungan `ANTHROPIC_MODEL` hanya berlaku untuk sesi yang Anda luncurkan dengan mereka. Untuk menjalankan model yang berbeda di terminal yang berbeda pada waktu yang sama, luncurkan masing-masing dengan bendera `--model` miliknya sendiri daripada beralih dengan `/model`.

Harga di pemilih `/model` muncul ketika Claude Code berbicara dengan Anthropic API, secara langsung atau melalui [gateway LLM](/docs/id/llm-gateway) yang memproksikannya, dan harga di baris adalah harga model yang dipilih baris tersebut. Di [penyedia pihak ketiga](/docs/id/third-party-integrations) seperti Amazon Bedrock dan di [gateway aplikasi Claude](/docs/id/claude-apps-gateway), penyedia atau gateway Anda menentukan apa yang Anda bayar, jadi baris pemilih tidak menampilkan harga. Harga adalah label tampilan saja; itu tidak mempengaruhi model mana yang dipilih baris atau apa yang ditagihkan penyedia Anda. Sebelum v2.1.206, [Claude Platform on AWS](/docs/id/claude-platform-on-aws) dan sesi gateway menampilkan harga daftar Anthropic, dan baris dapat menampilkan harga model yang berbeda dari yang dipilihnya.

Sesi yang dilanjutkan dimulai dengan `claude --resume`, `--continue`, atau pemilih `/resume` menyimpan model yang mereka gunakan ketika transkrip disimpan, terlepas dari pengaturan `model` saat ini. Jika model tersebut telah pensiun atau dikecualikan oleh [`availableModels`](#restrict-model-selection), sesi jatuh melalui urutan prioritas normal. Ini mencegah pilihan `/model` sesi lain dari mengubah model saat dilanjutkan.

Model yang Anda pilih untuk peluncuran baru dengan `--model` atau `ANTHROPIC_MODEL` masih memiliki prioritas lebih tinggi daripada model yang dipulihkan. Mulai dari v2.1.195, demikian juga variabel keluarga [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables).

Ketika model aktif saat startup berasal dari pengaturan proyek atau yang dikelola daripada pilihan Anda sendiri, header startup menunjukkan file pengaturan mana yang menetapkannya. Jalankan `/model` untuk mengganti; pengaturan proyek atau yang dikelola diterapkan kembali pada peluncuran berikutnya.

Ketika permintaan penggantian model dilakukan melalui metode [Agent SDK](/docs/id/agent-sdk/overview) `setModel()` atau oleh aplikasi seperti [Desktop app](/docs/id/desktop) yang menjalankan Claude Code CLI untuk Anda, Claude Code memeriksa bahwa string tersebut adalah salah satu yang dikenalinya sebelum menyimpannya. Pemeriksaan ini memerlukan Claude Code v2.1.200 atau lebih baru. Di Anthropic API, Claude Code mengenali:

* alias model
* entri dari pemilih `/model`
* nama apa pun yang dimulai dengan `claude-`
* nilai yang Anda konfigurasi sendiri sebagai [opsi model kustom](#add-a-custom-model-option) atau dalam [`modelOverrides`](#override-model-ids-per-version)

Claude Code menolak string yang tidak dikenali dengan `Model "<name>" is not a recognized model id.` dan sesi menyimpan model saat ini, daripada menyimpan string dan gagal pada permintaan berikutnya. Lihat [referensi kesalahan](/docs/id/errors#model-is-not-a-recognized-model-id) untuk langkah pemulihan.

Pemeriksaan hanya berjalan di Anthropic API. Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/id/claude-platform-on-aws), dan di belakang [gateway LLM](/docs/id/llm-gateway) atau `ANTHROPIC_BASE_URL` kustom, penyedia atau gateway Anda menentukan nama model, jadi Claude Code melewatkan string apa pun tanpa memeriksanya. Pemeriksaan juga tidak mencakup bendera `--model`, variabel lingkungan `ANTHROPIC_MODEL`, atau pengaturan `model`; nilai yang salah ketik di sana menghasilkan [There's an issue with the selected model](/docs/id/errors#theres-an-issue-with-the-selected-model) pada permintaan pertama sebagai gantinya.

Ketika model yang diminta memiliki tanggal pensiun yang dijadwalkan atau secara otomatis dipetakan ulang ke versi yang lebih baru, Claude Code menampilkan peringatan yang menyebutkan model yang diminta. Sesi interaktif menampilkannya sebagai pemberitahuan startup. Dari v2.1.182, peringatan yang sama ditulis ke stderr dalam [mode non-interaktif](/docs/id/headless) ketika menggunakan format output teks default. Pemeriksaan juga mencakup `model` yang ditetapkan dalam [frontmatter subagent](/docs/id/sub-agents). Peringatan stderr ditekan untuk `--output-format json` dan `stream-json`; baca model aktual dari bidang `modelUsage` dari [pesan hasil](/docs/id/headless#get-structured-output) sebagai gantinya.

Contoh penggunaan:

```bash theme={null}
# Mulai dengan Opus
claude --model opus

# Beralih ke Sonnet selama sesi
/model sonnet
```

Contoh file pengaturan:

```json theme={null}
{
    "permissions": {
        ...
    },
    "model": "opus"
}
```

<h2 id="restrict-model-selection">
  Batasi pemilihan model
</h2>

Administrator enterprise dapat menggunakan `availableModels` dalam [pengaturan terkelola atau kebijakan](/docs/id/settings#settings-files) untuk membatasi model mana yang dapat dipilih pengguna. Entri cocok dengan keluarga model seperti `sonnet`, awalan versi seperti `claude-sonnet-4-5`, atau ID model lengkap seperti `claude-sonnet-4-5-20250929`.

Ketika `availableModels` diatur, daftar izin berlaku di mana pun pengguna dapat menentukan model:

* **Model sesi utama**: `/model`, flag `--model`, variabel lingkungan `ANTHROPIC_MODEL`, pengaturan `model`, dan model yang dipulihkan ketika [melanjutkan sesi](#setting-your-model)
* **Resolusi alias**: variabel lingkungan `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, dan `ANTHROPIC_DEFAULT_FABLE_MODEL` tidak dapat mengarahkan ulang alias yang diizinkan ke model di luar daftar
* **Mode cepat**: `/fast` menolak untuk beralih ketika akan secara implisit beralih ke model Opus di luar daftar, dengan pesan "is not in your organization's allowed models"
* **Model subagent**: bidang `model` dalam frontmatter [subagent](/docs/id/sub-agents#choose-a-model), parameter `model` dari alat Agent, `CLAUDE_CODE_SUBAGENT_MODEL`, dan, pada v2.1.197 dan lebih awal, pemilih model di wizard `/agents`&#x20;
* **Model skill dan command**: frontmatter `model` dalam [skills dan commands](/docs/id/skills)
* **Model advisor**: pengaturan [`advisorModel`](/docs/id/advisor) yang dikonfigurasi dan flag `--advisor`
* **Model background agent**: model yang dipilih dalam [dispatch picker](/docs/id/agent-view)

Di API Anthropic dan [Claude Platform di AWS](/docs/id/claude-platform-on-aws), alias keluarga model, `opus`, `sonnet`, `haiku`, atau `fable`, diselesaikan ke versi terbaru dari keluarganya yang daftar izin izinkan. Ketika daftar izin menetapkan versi tertentu, misalnya `["sonnet", "claude-opus-4-6"]`, baik `/model opus` maupun `--model opus` memilih Claude Opus 4.6, Opus terbaru yang diizinkan, dan menampilkan pemberitahuan yang menyebutkan model yang diminta dan model pengganti. Sebelum v2.1.205, alias yang versi rilis terbarunya berada di luar daftar ditolak atau diganti seperti pilihan terblokir lainnya, bahkan ketika daftar mengizinkan versi yang lebih lama.

Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan [Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint) menggunakan ID penyebaran khusus penyedia daripada ID model Anthropic, jadi alias yang diblokir di sana mengikuti perilaku penolakan dan penggantian di bawah.

Claude Code menangani pilihan terblokir lainnya sesuai dengan tempat model diatur:

* **`/model`**: peralihan ditolak dengan kesalahan
* **Flag `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`**: nilai diganti saat startup dengan peringatan yang menyebutkan model yang diminta dan model pengganti, dan sesi dimulai pada model default
* **Penggantian subagent, skill, atau command**: penggantian kembali ke model yang diwariskan atau default daripada gagal dalam permintaan
* **Pengaturan `advisorModel`**: advisor dinonaktifkan untuk sesi
* **Flag `--advisor`**: Claude Code keluar dengan kesalahan saat peluncuran

Model yang dikecualikan disembunyikan dari pemilih `/model`. ID model lengkap dalam daftar yang tidak memiliki baris pemilih bawaan, seperti versi yang lebih lama yang ditetapkan daftar, muncul di pemilih `/model` sebagai baris berlabel miliknya sendiri. Sebelum v2.1.199, ID tersebut hanya dapat dipilih dengan mengetik `/model <id>`.

Perubahan model yang Claude Code buat atas nama Anda diperiksa dengan cara yang sama:

* **[Rantai model fallback](#fallback-model-chains)**: elemen di luar daftar izin dihapus
* **Peningkatan mode rencana**: di API Anthropic dan Claude Platform di AWS, peningkatan seperti [`opusplan`](#opusplan-model-setting) ke model yang dikecualikan menggunakan versi terbaru yang diizinkan dari keluarga peningkatan. Pada penyedia dengan ID model khusus penyedia, dan ketika tidak ada versi yang diizinkan, peningkatan dilewati dan perencanaan berlanjut pada model sesi
* **[Fallback model otomatis](#automatic-model-fallback)**: fallback yang targetnya dikecualikan tidak berjalan, sehingga permintaan yang ditandai berakhir dengan penolakan
* **[Mode cepat](/docs/id/fast-mode)**: mengaktifkan mode cepat ditolak ketika model yang akan dijalankan sesi setelahnya berada di luar daftar izin

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Cakupan permukaan
</h3>

Setiap permukaan memberlakukan daftar izin yang diterimanya. Mekanisme pengiriman mana yang mencapai setiap permukaan berbeda:

| Mekanisme pengiriman                                                         | CLI dan IDE  | Sesi lokal Desktop | Sesi web, mobile, dan cloud | Agent SDK dan non-interaktif | Cowork                          |
| :--------------------------------------------------------------------------- | :----------- | :----------------- | :-------------------------- | :--------------------------- | :------------------------------ |
| [Pengaturan terkelola server](/docs/id/server-managed-settings) dari konsol admin | Diberlakukan | Diberlakukan       | Diberlakukan                | Diberlakukan                 | Tidak dikirimkan                |
| [File pengaturan MDM atau terkelola](/docs/id/settings#settings-files)            | Diberlakukan | Diberlakukan       | Tidak dikirimkan            | Diberlakukan                 | Diberlakukan di mana diterapkan |

* Sesi cloud, di [Claude Code di web](/docs/id/claude-code-on-the-web) atau di aplikasi Desktop, berjalan di VM yang dikelola Anthropic: pengaturan yang diterapkan ke perangkat Anda tidak mencapainya, jadi kirimkan daftar izin melalui pengaturan terkelola server. Peralihan model pertengahan sesi dalam sesi cloud ditolak ketika model yang diminta dikecualikan oleh daftar izin. Penolakan sisi server saat pembuatan sesi berlaku untuk [pembatasan model organisasi](#organization-model-restrictions), bukan kunci pengaturan `availableModels`.
* Cowork, tab pekerjaan agentic di aplikasi Claude Desktop, bukan permukaan Claude Code dan tidak menerima pengaturan terkelola server sesuai desain. File pengaturan terkelola berlaku untuk sesi Cowork ketika ada di mana sesi berjalan; sesi Cowork jarak jauh berjalan di VM yang dikelola Anthropic, di mana file yang diterapkan perangkat tidak ada.
* Sesi di [penyedia pihak ketiga](/docs/id/server-managed-settings#platform-availability) seperti Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan [Claude Platform di AWS](/docs/id/claude-platform-on-aws) tidak menerima pengaturan terkelola server, jadi kirimkan daftar izin melalui file pengaturan MDM atau terkelola di sana.
* Pengiriman terkelola server juga memerlukan sesi untuk mengautentikasi dengan login organisasi atau kunci API yang dikonfigurasi langsung. Fleet yang menghasilkan kunci hanya melalui skrip [`apiKeyHelper`](/docs/id/settings#available-settings) harus mengirimkan daftar izin melalui file pengaturan MDM atau terkelola.
* Tab Desktop Code juga menampilkan [sesi SSH](/docs/id/desktop#ssh-sessions), yang membaca file pengaturan terkelola dari host jarak jauh tempat mereka berjalan. Lihat [Pengaturan terkelola Desktop](/docs/id/desktop#managed-settings).
* Pemilih model di claude.ai dan di aplikasi Desktop menyembunyikan atau memudarkan model yang dikecualikan oleh daftar izin organisasi Anda. Status pemilih adalah kenyamanan bagi pengguna; penegakan terjadi dalam sesi.

<h3 id="default-model-behavior">
  Perilaku model default
</h3>

Opsi Default di pemilih model tidak dipengaruhi oleh `availableModels` kecuali [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) juga diatur. Dengan sendirinya, `availableModels` membiarkan Default tersedia, diselesaikan ke [default runtime sistem](#default-model-setting) untuk akun. Jika default itu adalah model yang ingin Anda batasi, atur `enforceAvailableModels` juga.

Array `availableModels` yang kosong tidak pernah melibatkan penegakan Default-model: dengan `availableModels: []`, pilihan model bernama diblokir tetapi model Default untuk jenis akun tetap dapat digunakan terlepas dari `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Berlakukan daftar izin untuk model Default
</h3>

Atur `enforceAvailableModels: true` bersama `availableModels` yang tidak kosong dalam pengaturan terkelola untuk memperluas daftar izin ke opsi Default. Ini memerlukan Claude Code v2.1.175 atau lebih baru.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Opsi Default diselesaikan ke akun-jenis default, atau ke [model default organisasi](#organization-default-model) ketika admin telah menetapkan satu. Ketika model itu tidak ada dalam daftar izin, opsi Default malah diselesaikan ke entri pertama `availableModels` yang menamai model yang diizinkan dan tersedia, dan baris Default pemilih `/model` menunjukkan model tersebut. Ini berlaku di mana pun default dicapai: startup sesi, memilih Default di `/model`, kata kunci `"default"` dalam [rantai model fallback](#fallback-model-chains), dan fallback yang digunakan ketika pilihan yang dikecualikan dihapus.

`enforceAvailableModels` tidak berpengaruh ketika `availableModels` tidak diatur atau kosong: dengan `availableModels: []`, model Default untuk jenis akun tetap dapat digunakan, sehingga pengaturan tidak dapat mengunci pengguna dari setiap model. Ketika `availableModels` tidak kosong tetapi tidak ada entri yang diselesaikan ke model yang diizinkan dan tersedia, penegakan menurun dan Default jatuh kembali ke default jenis akun, dengan peringatan yang hanya terlihat di bawah `--debug`. Pertahankan setidaknya satu entri yang dijamin tersedia dalam daftar untuk menghindari ini.

Terapkan kedua kunci dalam [sumber terkelola dengan prioritas tertinggi](/docs/id/settings#settings-precedence): sumber terkelola yang diterapkan admin tidak menggabung, jadi pasangan yang ditempatkan dalam file pengaturan terkelola diabaikan ketika konsol admin mengirimkan pengaturan apa pun.

<h3 id="control-the-model-users-run-on">
  Kontrol model yang dijalankan pengguna
</h3>

Pengaturan `model` adalah pilihan awal, bukan penegakan. Ini menetapkan model mana yang aktif ketika sesi dimulai, tetapi pengguna masih dapat membuka `/model` dan memilih Default, yang diselesaikan ke default sistem untuk [runtime default](#default-model-setting) terlepas dari apa yang `model` diatur, kecuali [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) mengarahkannya ulang.

Untuk sepenuhnya mengontrol pengalaman model, gabungkan pengaturan ini:

* **`availableModels`**: membatasi model bernama mana yang dapat dialihkan pengguna
* **`enforceAvailableModels`**: memperluas daftar izin `availableModels` ke opsi Default, sehingga Default tidak dapat diselesaikan ke model di luar daftar
* **`model`**: menetapkan pilihan model awal ketika sesi dimulai
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: mengontrol apa yang diselesaikan opsi Default dan alias `sonnet`, `opus`, `haiku`, dan `fable`

Contoh ini memulai pengguna di Sonnet 4.5, membatasi pemilih ke Sonnet dan Haiku, dan memastikan Default diselesaikan ke model dalam daftar izin daripada default tingkat:

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Tanpa `enforceAvailableModels` atau blok `env`, pengguna yang memilih Default di pemilih akan mendapatkan rilis terbaru untuk tingkat mereka, melewati pin versi dalam `model` dan `availableModels`. Dua pengaturan mencakup cakupan yang berbeda: `enforceAvailableModels` membuat Default mematuhi daftar izin, sementara blok `env` menetapkan versi mana yang diselesaikan alias yang diizinkan seperti `sonnet`. Gunakan `enforceAvailableModels` saja ketika membatasi keluarga model sudah cukup; tambahkan blok `env` ketika Anda juga perlu menetapkan versi tertentu.

<h3 id="merge-behavior">
  Perilaku penggabungan
</h3>

Ketika [sumber pengaturan terkelola dengan prioritas tertinggi](/docs/id/settings#settings-precedence) mendefinisikan `availableModels`, hanya daftar itu yang berlaku: entri dalam pengaturan pengguna, proyek, atau lokal tidak dapat memperluas daftar, dan sumber terkelola yang diterapkan admin tidak menggabung satu sama lain, jadi daftar yang diterapkan dalam file pengaturan terkelola diabaikan ketika pengaturan terkelola server mengirimkan kunci apa pun. Jika tidak, daftar dari pengaturan pengguna, proyek, dan lokal [digabungkan dan dideduplikasi](/docs/id/settings#settings-precedence) seperti pengaturan array lainnya. Mulai dari Claude Code v2.1.175, daftar terkelola menggantikan entri prioritas lebih rendah; versi sebelumnya menggabungkan mereka.

Dalam daftar yang efektif, entri yang menamai model tertentu dalam keluarga, baik awalan versi atau ID model lengkap, menonaktifkan entri wildcard keluarga itu: `["sonnet", "claude-sonnet-4-5"]` hanya memungkinkan versi Sonnet 4.5, bukan setiap model Sonnet.

<h3 id="mantle-model-ids">
  ID model Mantle
</h3>

Ketika [endpoint Bedrock Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint) diaktifkan, entri dalam `availableModels` yang dimulai dengan `anthropic.` ditambahkan ke pemilih `/model` sebagai opsi kustom dan dirutekan ke endpoint Mantle. Ini adalah pengecualian terhadap pencocokan alias yang dijelaskan dalam [Pin models for third-party deployments](#pin-models-for-third-party-deployments). Pengaturan masih membatasi pemilih ke entri yang tercantum, dan ID Mantle menyematkan nama keluarga, jadi dihitung sebagai entri tertentu dan menonaktifkan wildcard keluarga itu: bersama ID Mantle apa pun, daftarkan awalan versi atau ID lengkap yang ingin Anda pertahankan dapat dipilih. Lihat [Perilaku penggabungan](#merge-behavior).

<h3 id="organization-model-restrictions">
  Pembatasan model organisasi
</h3>

Admin organisasi pada paket Claude Enterprise membatasi model mana yang dapat dijalankan anggota dengan menonaktifkan model individual di konsol admin claude.ai. Pembatasan ini dikirimkan dengan hak akses akun ketika Claude Code mengautentikasi, terpisah dari daftar `availableModels` apa pun dalam pengaturan, dan server memberlakukan pembatasan yang sama secara independen ketika sesi dibuat. Memerlukan Claude Code v2.1.187 atau lebih baru.

Pembatasan berlaku ketika anggota masuk atau menggunakan kunci API mereka sendiri. Kredensial yang bersifat organisasi, seperti kunci layanan organisasi, tidak terikat pada pengguna, jadi pembatasan tidak berlaku untuk mereka.

Claude Console tidak memiliki kontrol pembatasan model. Organisasi tanpa paket Claude Enterprise, termasuk mereka yang anggotanya mengautentikasi melalui API Anthropic, membatasi model dengan [`availableModels`](#restrict-model-selection) dalam [pengaturan terkelola](/docs/id/settings#settings-files) sebagai gantinya, menambahkan [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) untuk mencakup opsi Default. Pengaturan ini diberlakukan oleh Claude Code itu sendiri, bukan oleh server.

Model yang dibatasi disembunyikan dari pemilih `/model`. Memilihnya berdasarkan nama dengan `--model`, variabel lingkungan `ANTHROPIC_MODEL`, atau pengaturan `model` menunjukkan pemberitahuan `Model "<name>" is restricted by your organization's settings. Using <model> instead.` dan sesi dimulai pada model yang diizinkan. Mengetik `/model <name>` untuk model yang dibatasi ditolak dengan `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` dan sesi mempertahankan model saat ini.

Alias [keluarga model](#restrict-model-selection) seperti `opus` diselesaikan ke versi terbaru dari keluarganya yang organisasi izinkan, dengan pemberitahuan penggantian yang sama. `/model <alias>` ditolak hanya ketika setiap versi keluarganya dibatasi; alias yang diatur dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model` masih diganti saat startup dalam hal itu. Sebelum v2.1.205, alias keluarga diganti atau ditolak berdasarkan versi rilis terbarunya saja, bahkan ketika versi yang lebih lama diizinkan.

Pembatasan berlaku org-wide atau per role:

* Menonaktifkan model di tingkat organisasi menghapusnya untuk setiap anggota.
* Akses tingkat role memberikan model yang berbeda ke role kustom yang berbeda, dan anggota yang memiliki beberapa role dapat menggunakan model apa pun yang diizinkan salah satu role mereka.
* Model Haiku selalu tersedia dan tidak dapat dinonaktifkan, jadi setiap anggota menyimpan setidaknya satu model yang dapat digunakan.
* Perubahan akses berlaku org-wide dalam sekitar satu menit; pemilih `/model` mencerminkannya saat sesi berikutnya dimulai.

Kedua pembatasan berlaku bersama: model dapat dipilih hanya ketika diizinkan oleh `availableModels` dan tidak dibatasi oleh organisasi. Pembatasan organisasi dikirimkan ke sesi di API Anthropic dan penyebaran [gateway LLM](/docs/id/llm-gateway). Sesi di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan Claude Platform di AWS tidak menerimanya, jadi gunakan `availableModels` di penyedia tersebut.

<h2 id="organization-default-model">
  Model default organisasi
</h2>

Admin organisasi pada paket Claude Enterprise dapat menetapkan model default untuk anggota Claude Code dari konsol admin claude.ai, untuk seluruh organisasi atau per role kustom. Ketika satu diatur, opsi Default diselesaikan ke model itu alih-alih [default jenis akun](#default-model-setting). Memerlukan Claude Code v2.1.196 atau lebih baru.

Baris Default di pemilih `/model` menampilkan nama model default organisasi dengan label Org default. Label membaca Org default apakah admin menetapkan default untuk seluruh organisasi atau untuk role Anda. Default role mencakup anggota role kustom itu dan mengambil alih default organisasi-lebar; ketika beberapa role Anda menetapkan default yang berbeda, model paling mampu berlaku.

Model default organisasi adalah titik awal, bukan pembatasan, dan pilihan model lainnya mengambil alih atasnya:

* flag `--model` dan variabel lingkungan `ANTHROPIC_MODEL`
* nilai `model` dalam [pengaturan terkelola](/docs/id/settings#settings-files) atau disediakan melalui `--settings`
* nilai `model` dalam pengaturan pengguna, proyek, atau lokal Anda, termasuk model yang Anda simpan dengan `/model`

Admin juga dapat mengonfigurasi model default organisasi untuk mengganti pilihan pengguna. Dengan override aktif, ia mengambil alih nilai `model` dalam pengaturan pengguna, proyek, dan lokal, jadi model yang Anda simpan dengan `/model` berlaku untuk sesi saat ini dan model default organisasi kembali pada peluncuran berikutnya. Ketika pilihan Anda berbeda, `/model` menampilkan `Your organization's default (<model>) applies on restart`. Flag `--model`, `ANTHROPIC_MODEL`, pengaturan terkelola, dan `--settings` masih mengambil alih bahkan dengan override aktif. Override tersedia untuk set organisasi terbatas; tanyakan tim akun Anthropic Anda tentang ketersediaan.

Untuk membatasi model mana yang dapat dipilih anggota, gunakan [pembatasan model organisasi](#organization-model-restrictions) atau [`availableModels`](#restrict-model-selection) sebagai gantinya.

Claude Code membaca model default organisasi sekali saat startup, jadi default yang diubah admin pertengahan sesi berlaku pada peluncuran berikutnya.

Ketika model default organisasi tidak mengganti pilihan pengguna, peluncuran interaktif pertama setelah admin mengubahnya menghapus kunci `model` dari pengaturan pengguna Anda sekali, sehingga default baru berlaku. Ini tidak mengubah apa pun lagi dalam file, dan model yang Anda simpan dengan `/model` setelah peluncuran itu disimpan.

Model default organisasi melewati pemeriksaan pembatasan yang sama seperti model Default lainnya sebelum diadopsi:

* [`availableModels`](#restrict-model-selection) dengan sendirinya tidak pernah membatasi opsi Default, jadi model default organisasi di luar daftar izin masih berlaku. Ketika [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) juga diatur, model default organisasi di luar daftar izin dipetakan ulang ke entri daftar izin pertama, seperti Default lainnya
* model default organisasi yang [pembatasan model organisasi](#organization-model-restrictions) tolak untuk akun Anda diganti dengan model yang paling baru diizinkan dalam keluarganya, atau keluarga biaya lebih rendah ketika setiap versi itu dibatasi
* model default organisasi yang tidak tersedia untuk akun Anda sama sekali, seperti Fable 5 di bawah [zero data retention](/docs/id/zero-data-retention), dilewati, dan opsi Default diselesaikan ke default jenis akun

Mulai dari v2.1.199, ketika model default organisasi adalah keluarga model yang berbeda dari default biasa jenis akun Anda, pemilih `/model` menyimpan baris terpisah untuk keluarga biasa itu, sehingga Anda masih dapat beralih ke itu untuk sesi. Di v2.1.196 hingga v2.1.198 baris itu hilang dari pemilih.

Model default organisasi dikirimkan ke sesi yang diautentikasi dengan API Anthropic. Sesi di penyebaran [gateway LLM](/docs/id/llm-gateway), Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan Claude Platform di AWS tidak menerimanya. Untuk menetapkan default pada penyebaran itu, gunakan kunci `model` dalam [pengaturan terkelola](/docs/id/settings#settings-files) sebagai gantinya.

<h2 id="organization-effort-limits">
  Batas usaha organisasi
</h2>

Admin organisasi pada paket Claude Enterprise dapat menetapkan tingkat [usaha](#adjust-effort-level) maksimum per model untuk setiap role kustom, bersama [pembatasan model organisasi](#organization-model-restrictions) tingkat role. Tingkat di atas batas tidak ditawarkan di pemilih `/effort`, dan menamai tingkat lebih tinggi dengan `--effort` atau `/effort` berjalan pada batas sebagai gantinya. Dalam sesi interaktif dan lari `--print` teks biasa, peringatan menyebutkan tingkat yang diminta dan diterapkan; dengan output `json` atau `stream-json` atau di agen latar belakang, penjepit berlaku diam-diam. Batas adalah per model, jadi beralih model dapat mengubah tingkat mana yang tersedia. Ketika beberapa role Anda memberikan model yang sama, batas paling permisif berlaku. Memerlukan Claude Code v2.1.195 atau lebih baru.

Batas usaha dikirimkan bersama [pembatasan model organisasi](#organization-model-restrictions) dan mengikuti ketersediaan penyedia yang sama: sesi di Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, dan Claude Platform di AWS tidak menerimanya.

<h2 id="special-model-behavior">
  Perilaku model khusus
</h2>

<h3 id="default-model-setting">
  Pengaturan model `default`
</h3>

Perilaku `default` tergantung pada jenis akun Anda:

* **Max, Team Premium, Enterprise pay-as-you-go, dan Anthropic API**: default ke Opus 4.8
* **Claude Platform di AWS, Amazon Bedrock, dan Google Cloud's Agent Platform**: default ke Opus 4.8
* **Pro, Team Standard, dan kursi langganan Enterprise**: default ke Sonnet 5
* **Microsoft Foundry**: default ke Sonnet 4.5

Enterprise pay-as-you-go berarti organisasi Enterprise yang ditagihkan berdasarkan penggunaan daripada kursi langganan.

Sebelum v2.1.207, `default` diselesaikan ke Opus 4.7 di Claude Platform di AWS dan ke Sonnet 4.5 di Amazon Bedrock dan Google Cloud's Agent Platform.

Ketika admin telah menetapkan [model default organisasi](#organization-default-model), `default` diselesaikan ke model itu sebagai gantinya dari default jenis akun di atas. Memerlukan Claude Code v2.1.196 atau lebih baru.

Ketika pengaturan terkelola [memberlakukan allowlist untuk model Default](#enforce-the-allowlist-for-the-default-model) dan default jenis akun tidak ada dalam `availableModels`, `default` diselesaikan ke Default yang diberlakukan daripada default jenis akun di atas. Ketika keduanya berlaku, model default organisasi menggantikan default jenis akun terlebih dahulu dan penegakan kemudian berlaku untuk itu: model default organisasi yang diizinkan disimpan, sementara yang di luar daftar diselesaikan ke Default yang diberlakukan.

Fable 5 bukan model default pada jenis akun apa pun. Sesi menggunakan Fable 5 hanya setelah Anda memilihnya, dengan `/model fable`, pengaturan `model`, atau alias `best` di mana Fable 5 tersedia. Memilihnya dengan `/model` menyimpannya sebagai model yang dipilih dalam pengaturan pengguna Anda, sehingga sesi berikutnya dimulai pada Fable 5 sampai Anda mengubah model.

<h3 id="opusplan-model-setting">
  Pengaturan model `opusplan`
</h3>

Alias model `opusplan` menyediakan pendekatan hibrida otomatis:

* **Dalam plan mode**: menggunakan `opus` untuk penalaran kompleks dan keputusan arsitektur
* **Dalam execution mode**: secara otomatis beralih ke `sonnet` untuk pembuatan kode dan implementasi

Ini menggabungkan penalaran Opus untuk perencanaan dengan efisiensi Sonnet untuk eksekusi.

Fase Opus dalam plan mode menggunakan jendela konteks yang sama dengan pengaturan model `opus`. Pada tingkat langganan di mana Opus [secara otomatis ditingkatkan ke konteks 1M](#extended-context), `opusplan` menerima peningkatan dalam plan mode juga. Untuk memaksa konteks 1M untuk kedua fase ketika Anda tidak berada di tingkat auto-upgrade, atur model ke `opusplan[1m]`.

Ketika [`availableModels`](#restrict-model-selection) mengecualikan Opus terbaru tetapi mengizinkan versi yang lebih lama, misalnya `["sonnet", "claude-opus-4-6"]`, `opusplan` menggunakan Opus terbaru yang diizinkan untuk perencanaan dan tetap pada Sonnet hanya ketika setiap Opus dikecualikan. Sesi Haiku yang biasanya akan ditingkatkan ke Sonnet dalam plan mode demikian pula menggunakan Sonnet terbaru yang diizinkan, dan tetap pada Haiku hanya ketika setiap Sonnet dikecualikan. Sebelum v2.1.205, plan mode tetap pada model sesi kapan pun versi terbaru dari keluarga upgrade dikecualikan, bahkan ketika allowlist mengizinkan yang lebih lama.

Substitusi versi yang lebih lama yang diizinkan berlaku pada Anthropic API dan [Claude Platform di AWS](/docs/id/claude-platform-on-aws). Pada Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan Mantle, yang penyebarannya menggunakan ID model spesifik penyedia, plan mode tetap pada model sesi kapan pun model upgrade dikecualikan.

Untuk pendekatan hibrida di mana Claude memutuskan di tengah-tugas kapan harus berkonsultasi dengan model kedua daripada beralih di batas rencana, lihat [advisor tool](/docs/id/advisor).

<h3 id="fallback-model-chains">
  Rantai model fallback
</h3>

Ketika model utama kelebihan beban, tidak tersedia, atau mengembalikan kesalahan server yang tidak dapat dicoba ulang lainnya, Claude Code dapat beralih ke model fallback daripada gagal permintaan. Kesalahan autentikasi, penagihan, batas laju, ukuran permintaan, dan transportasi tidak pernah memicu switch; mereka mengikuti retry dan penanganan kesalahan normal mereka.

Konfigurasikan satu atau lebih model fallback dan Claude Code mencobanya secara berurutan, menampilkan pemberitahuan saat beralih. Switch berlangsung hanya untuk giliran saat ini, jadi pesan berikutnya Anda mencoba model utama terlebih dahulu lagi. Chain dibatasi pada tiga model setelah penghapusan duplikat, dan entri tambahan diabaikan.

Atur chain untuk satu sesi dengan flag `--fallback-model`, yang menerima daftar yang dipisahkan koma:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Untuk mempertahankan chain di seluruh sesi, atur `fallbackModel` dalam [settings](/docs/id/settings) sebagai array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Flag `--fallback-model` mengambil alih pengaturan `fallbackModel`. Setiap elemen menerima nama model atau alias, dan `"default"` berkembang menjadi model default.

Dua kasus menyebabkan elemen dilewati:

* **Unavailable model**: model yang tidak dapat dijangkau, seperti model yang pensiun yang disematkan dalam pengaturan, dilewati dan Claude Code melanjutkan ke elemen berikutnya.
* **Outside the allowlist**: elemen yang tidak diizinkan oleh [`availableModels`](#restrict-model-selection) dijatuhkan saat chain dibaca dan tidak pernah dicoba.

<h3 id="automatic-model-fallback">
  Fallback model otomatis
</h3>

Bagian ini mencakup fallback berbasis konten dari Fable 5. Untuk fallback berbasis ketersediaan ketika model kelebihan beban atau tidak tersedia, lihat [Rantai model fallback](#fallback-model-chains).

Fable 5 berjalan dengan pengklasifikasi keamanan untuk konten keamanan siber dan biologi. Ketika pengklasifikasi menandai permintaan, Claude Code menjalankan kembali permintaan itu pada model Opus default penyedia Anda dan menampilkan pemberitahuan dalam transkrip. Pada Anthropic API, penyebaran [LLM gateway](/docs/id/llm-gateway), dan [Claude Platform di AWS](/docs/id/claude-platform-on-aws), model itu adalah Opus 4.8. Pada [Claude apps gateway](/docs/id/claude-apps-gateway), itu adalah Opus 4.7 kecuali Anda menunjukkan alias [`opus`](#environment-variables) ke model lain.

Sesi kemudian berlanjut pada model Opus itu. Untuk kembali ke Fable 5, jalankan `/model fable`.

Target fallback diperiksa terhadap [`availableModels`](#restrict-model-selection). Ketika diblokir, tidak ada fallback yang terjadi. Penolakan muncul sebagai kesalahan normal dan model sesi tidak berubah.

<h4 id="check-what-triggered-fallback">
  Periksa apa yang memicu fallback
</h4>

Fallback dapat memicu pada permintaan pertama sesi, sebelum Anda mengirim apa pun yang tidak biasa, karena permintaan pertama membawa konteks workspace seperti konten CLAUDE.md dan status git Anda. Repositori yang berisi materi keamanan atau biologi dapat memicu pengklasifikasi pada konteks itu saja.

Untuk memeriksa apakah kustomisasi adalah pemicu, mulai sesi dengan `claude --safe-mode`, yang menonaktifkan kustomisasi seperti CLAUDE.md, skills, MCP servers, dan hooks. Status git dan nama direktori bukan kustomisasi dan masih disertakan.

<h4 id="ask-before-switching">
  Tanyakan sebelum beralih
</h4>

Untuk memutuskan apa yang terjadi setiap kali permintaan ditandai, daripada beralih secara otomatis, jalankan `/config` dan matikan "switch models when a message is flagged". Permintaan yang ditandai kemudian menjeda sesi dengan dua opsi: beralih ke model Opus, atau edit prompt dan coba ulang pada Fable 5.

Beberapa kasus berperilaku berbeda:

* Jika kedua model menandai permintaan yang sama, Anda dapat mengedit prompt dan mencoba ulang, atau memulai sesi baru.
* Pada sesi mobile [Claude Code di web](/docs/id/claude-code-on-the-web), pengeditan dan pengulangan tidak didukung. Beralih model, atau lanjutkan sesi dari browser desktop atau aplikasi desktop.
* Dalam [mode non-interaktif](/docs/id/cli-reference#cli-flags) dan integrasi SDK yang tidak dapat menampilkan prompt, permintaan yang ditandai mengakhiri giliran dengan penolakan sebagai gantinya.
* Ketika target fallback diblokir oleh [`availableModels`](#restrict-model-selection), prompt tidak ditampilkan. Permintaan yang ditandai berakhir dengan penolakan, sama seperti fallback otomatis ketika target diblokir.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Aktifkan fallback di Bedrock, Agent Platform, dan Foundry
</h4>

Pada [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), dan [Microsoft Foundry](/docs/id/microsoft-foundry), ID model spesifik penyedia, jadi fallback otomatis hanya beroperasi ketika Claude Code dapat mengidentifikasi kedua model yang terlibat:

* Claude Code harus mengenali model saat ini sebagai Fable 5: ID model berisi `claude-fable-5`, cocok dengan nilai `ANTHROPIC_DEFAULT_FABLE_MODEL`, atau dipetakan dengan [`modelOverrides`](#override-model-ids-per-version).
* Target fallback harus diselesaikan ke model Opus: nilai `ANTHROPIC_DEFAULT_OPUS_MODEL` jika diatur, jika tidak, entri Opus 4.8 dalam daftar model penyedia.

Jika salah satu model tidak dapat diidentifikasi, Claude Code tidak beralih secara otomatis. Permintaan yang ditandai berakhir dengan pesan penolakan, dan Anda dapat beralih model dengan [`/model`](#setting-your-model) dan coba ulang. Untuk mengaktifkan fallback otomatis pada penyedia ini, atur `ANTHROPIC_DEFAULT_FABLE_MODEL` ke ID model Fable 5 Anda dan `ANTHROPIC_DEFAULT_OPUS_MODEL` ke ID model Opus 4.8 Anda.

<h4 id="security-research-and-biology-workloads">
  Beban kerja penelitian keamanan dan biologi
</h4>

Beban kerja dalam keamanan ofensif atau biologi, termasuk penetration testing, latihan Capture the Flag (CTF), dan basis kode yang berdekatan dengan biologi, memicu fallback sering, sering pada permintaan pertama. Untuk pekerjaan biologi substansial, harapkan hampir semua permintaan untuk dialihkan.

Ini adalah routing yang diharapkan untuk domain ini, bukan bendera akun. Jika organisasi Anda membutuhkan kemampuan kelas Fable untuk pekerjaan ini, tanyakan kepada tim akun Anthropic Anda tentang program akses terpercaya.

<h3 id="adjust-effort-level">
  Sesuaikan tingkat usaha
</h3>

[Tingkat usaha](https://platform.claude.com/docs/en/build-with-claude/effort) mengontrol penalaran adaptif, yang memungkinkan model memutuskan apakah dan berapa banyak untuk berpikir pada setiap langkah berdasarkan kompleksitas tugas. Usaha lebih rendah lebih cepat dan lebih murah untuk tugas-tugas langsung, sementara usaha lebih tinggi memberikan penalaran lebih dalam untuk masalah kompleks.

Tingkat usaha yang tersedia tergantung pada model. Model yang tidak tercantum di sini tidak mendukung usaha:

| Model                            | Levels                                  |
| :------------------------------- | :-------------------------------------- |
| Fable 5                          | `low`, `medium`, `high`, `xhigh`, `max` |
| Sonnet 5, Opus 4.8, dan Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 dan Sonnet 4.6          | `low`, `medium`, `high`, `max`          |

Jika Anda menetapkan tingkat yang tidak didukung model aktif, Claude Code kembali ke tingkat tertinggi yang didukung pada atau di bawah tingkat yang Anda tetapkan. Misalnya, `xhigh` berjalan sebagai `high` pada Opus 4.6. Organisasi Anda juga dapat membatasi tingkat mana yang tersedia untuk model; lihat [Batas usaha organisasi](#organization-effort-limits).

Usaha default adalah `high` pada Fable 5, Sonnet 5, Opus 4.8, Opus 4.6, dan Sonnet 4.6, dan `xhigh` pada Opus 4.7.

Ketika Anda pertama kali menjalankan Fable 5, Opus 4.8, atau Opus 4.7, Claude Code menerapkan usaha default model itu bahkan jika Anda sebelumnya menetapkan tingkat yang berbeda untuk model lain: `high` pada Fable 5 dan Opus 4.8, dan `xhigh` pada Opus 4.7. Jalankan `/effort` lagi untuk memilih tingkat yang berbeda setelah beralih. Default itu dipertahankan di seluruh sesi sampai Anda membuat pilihan usaha eksplisit, seperti menjalankan `/effort` dalam sesi interaktif atau meluncurkan dengan `--effort`.

`low`, `medium`, `high`, dan `xhigh` bertahan di seluruh sesi ketika Anda menetapkannya dalam sesi interaktif. Tingkat yang ditetapkan dengan `/effort` dalam [mode non-interaktif](/docs/id/headless), dengan flag `-p`, berlaku untuk sesi saat ini saja dan tidak disimpan sebagai default Anda. `/effort` non-interaktif juga tidak dapat melepaskan penahan default model di atas: pada Fable 5, Opus 4.8, dan Opus 4.7 itu melaporkan `Not applied` dan sesi tetap pada usaha default model, jadi teruskan `--effort` saat peluncuran sebagai gantinya. `max` memberikan penalaran paling dalam tanpa batasan pengeluaran token dan berlaku untuk sesi saat ini saja, kecuali ketika diatur melalui variabel lingkungan `CLAUDE_CODE_EFFORT_LEVEL`.

Menu `/effort` juga menawarkan `ultracode`. Ultracode adalah pengaturan Claude Code daripada tingkat usaha model: ia mengirim `xhigh` ke model dan selain itu memiliki Claude mengorkestra [alur kerja dinamis](/docs/id/workflows) untuk tugas-tugas substansial. Ini berlaku untuk sesi saat ini saja.

Anda dapat mengaktifkan ultracode melalui salah satu dari berikut ini:

* **`/effort`**: jalankan `/effort ultracode`, atau pilih dari menu
* **Flag `--effort`**: luncurkan dengan `claude --effort ultracode`, yang memulai sesi pada usaha `xhigh` dengan ultracode aktif
* **`--settings` atau permintaan kontrol Agent SDK**: teruskan `"ultracode": true`. Permintaan [`applyFlagSettings()`](/docs/id/agent-sdk/typescript#applyflagsettings) juga menerima `effortLevel: "ultracode"`

Melewatkan `ultracode` ke flag `--effort` atau nilai Agent SDK `effortLevel` memerlukan Claude Code v2.1.203 atau lebih baru. Sebelum v2.1.203, `--effort ultracode` mencetak `Unknown --effort value 'ultracode'` dan sesi dimulai pada usaha default.

Pengaturan `effortLevel` yang bertahan dan variabel lingkungan `CLAUDE_CODE_EFFORT_LEVEL` tidak menerima `ultracode`.

Ketika ultracode tidak tersedia, misalnya ketika [workflows dimatikan](/docs/id/workflows#turn-workflows-off), `--effort ultracode` menetapkan usaha `xhigh` saja.

<h4 id="choose-an-effort-level">
  Pilih tingkat usaha
</h4>

Setiap tingkat menukar pengeluaran token terhadap kemampuan. Default cocok untuk sebagian besar tugas coding; sesuaikan ketika Anda menginginkan keseimbangan yang berbeda.

| Level       | Kapan menggunakannya                                                                                                                                                     |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Cadangkan untuk tugas pendek, terbatas, sensitif latensi yang tidak sensitif intelijen                                                                                   |
| `medium`    | Mengurangi penggunaan token untuk pekerjaan sensitif biaya yang dapat menukar beberapa intelijen                                                                         |
| `high`      | Menyeimbangkan penggunaan token dan intelijen. Default pada Fable 5, Sonnet 5, Opus 4.8, Opus 4.6, dan Sonnet 4.6                                                        |
| `xhigh`     | Penalaran lebih dalam dengan pengeluaran token lebih tinggi. Default pada Opus 4.7                                                                                       |
| `max`       | Dapat meningkatkan kinerja pada tugas yang menuntut tetapi mungkin menunjukkan hasil yang berkurang dan rentan terhadap overthinking. Uji sebelum mengadopsi secara luas |
| `ultracode` | Pengaturan Claude Code yang merencanakan [alur kerja dinamis](/docs/id/workflows) untuk setiap tugas substansial dengan penalaran `xhigh` per-pesan. Hanya sesi               |

Skala usaha dikalibrasi per model, jadi nama tingkat yang sama tidak mewakili nilai yang sama di seluruh model.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Gunakan ultrathink untuk penalaran mendalam sekali jalan
</h4>

Sertakan `ultrathink` di mana saja dalam prompt Anda untuk meminta penalaran lebih dalam pada giliran itu tanpa mengubah pengaturan usaha sesi Anda. Claude Code mengenali kata kunci dan menambahkan instruksi dalam konteks. Tingkat usaha yang dikirim ke API tidak berubah. Frasa lain seperti "think", "think hard", dan "think more" dilewatkan sebagai teks prompt biasa dan tidak dikenali sebagai kata kunci.

<h4 id="set-the-effort-level">
  Atur tingkat usaha
</h4>

Anda dapat mengubah usaha melalui salah satu dari berikut ini:

* **`/effort`**: jalankan `/effort` tanpa argumen untuk membuka slider interaktif, `/effort` diikuti dengan nama tingkat untuk menetapkannya secara langsung, atau `/effort auto` untuk mengatur ulang ke default model
* **Dalam `/model`**: gunakan tombol panah kiri/kanan untuk menyesuaikan slider usaha saat memilih model
* **Flag `--effort`**: teruskan nama tingkat untuk menetapkannya untuk sesi tunggal saat meluncurkan Claude Code
* **Variabel lingkungan**: atur `CLAUDE_CODE_EFFORT_LEVEL` ke nama tingkat atau `auto`
* **Pengaturan**: atur `effortLevel` ke `low`, `medium`, `high`, atau `xhigh` dalam file pengaturan Anda. `max` dan `ultracode` adalah [hanya sesi](#adjust-effort-level) dan tidak diterima di sini
* **Skill dan subagent frontmatter**: atur `effort` dalam file markdown [skill](/docs/id/skills#frontmatter-reference) atau [subagent](/docs/id/sub-agents#supported-frontmatter-fields) untuk mengganti tingkat usaha ketika skill atau subagent itu berjalan

Variabel lingkungan mengambil alih semua metode lain, kemudian tingkat yang Anda konfigurasi, kemudian default model. Usaha frontmatter berlaku ketika skill atau subagent itu aktif, mengganti tingkat sesi tetapi bukan variabel lingkungan.

Slider usaha muncul dalam `/model` ketika model yang didukung dipilih. Tingkat usaha saat ini juga ditampilkan di sebelah logo dan spinner, misalnya "with low effort", sehingga Anda dapat mengkonfirmasi pengaturan mana yang aktif tanpa membuka `/model`.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Penalaran adaptif dan anggaran pemikiran tetap
</h4>

Penalaran adaptif membuat pemikiran opsional pada setiap langkah, jadi Claude dapat merespons lebih cepat ke prompt rutin dan menyisihkan pemikiran lebih dalam untuk langkah yang mendapat manfaat darinya. Jika Anda ingin Claude berpikir lebih atau kurang sering daripada tingkat saat ini menghasilkan, Anda dapat mengatakan demikian secara langsung dalam prompt Anda atau dalam `CLAUDE.md`; model merespons panduan itu dalam pengaturan usahanya.

Fable 5, Sonnet 5, dan Opus 4.7 dan yang lebih baru selalu menggunakan penalaran adaptif. Mode anggaran pemikiran tetap dan `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` tidak berlaku untuk mereka.

Di Opus 4.6 dan Sonnet 4.6, Anda dapat mengatur `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` untuk kembali ke anggaran pemikiran tetap sebelumnya yang dikendalikan oleh `MAX_THINKING_TOKENS`. Lihat [variabel lingkungan](/docs/id/env-vars).

<h3 id="extended-thinking">
  Pemikiran diperluas
</h3>

Pemikiran diperluas adalah penalaran yang Claude keluarkan sebelum merespons. Pada model yang mendukung [penalaran adaptif](#adjust-effort-level), tingkat usaha adalah kontrol utama untuk berapa banyak pemikiran yang terjadi; pengaturan di bawah ini menghidupkan atau mematikan pemikiran dan mengontrol cara tampilannya.

| Control                         | Cara menetapkannya                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Toggle untuk sesi saat ini      | Tekan `Option+T` di macOS atau `Alt+T` di Windows dan Linux                                                                                                                                                                                                                                                                                                                                        |
| Atur default global             | Jalankan `/config` dan toggle thinking mode. Disimpan sebagai `alwaysThinkingEnabled` dalam `~/.claude/settings.json`                                                                                                                                                                                                                                                                              |
| Nonaktifkan terlepas dari usaha | Atur [`MAX_THINKING_TOKENS=0`](/docs/id/env-vars), yang mematikan pemikiran pada Anthropic API kecuali pada Fable 5. Pada [penyedia pihak ketiga](/docs/id/third-party-integrations) ini menghilangkan parameter `thinking` sebagai gantinya, dan model penalaran adaptif mungkin masih berpikir. Nilai lain berlaku hanya dengan [anggaran pemikiran tetap](#adaptive-reasoning-and-fixed-thinking-budgets) |

Pemikiran tidak dapat dimatikan pada Fable 5. Toggle sesi, `alwaysThinkingEnabled`, dan `MAX_THINKING_TOKENS=0` tidak memiliki efek di sana, dan Fable 5 memutuskan per langkah berapa banyak untuk berpikir berdasarkan tingkat usaha.

Output pemikiran dilipat secara default. Tekan `Ctrl+O` untuk toggle verbose mode dan lihat penalaran sebagai teks miring abu-abu. Sesi interaktif pada Anthropic API menerima blok pemikiran yang diredaksi secara default, jadi atur `showThinkingSummaries: true` dalam [pengaturan](/docs/id/settings) jika Anda menginginkan ringkasan lengkap yang tersedia saat Anda memperluas. Anda dikenakan biaya untuk semua token pemikiran yang dihasilkan, bahkan ketika dilipat atau diredaksi.

<h3 id="extended-context">
  Konteks diperluas
</h3>

Fable 5, Sonnet 5, Opus 4.6 dan yang lebih baru, dan Sonnet 4.6 mendukung [jendela konteks 1 juta token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) untuk sesi panjang dengan basis kode besar.

Ketersediaan bervariasi menurut model dan paket. Pada Anthropic API, Fable 5, Sonnet 5, Opus 4.8, dan Opus 4.7 selalu berjalan dengan jendela 1M. Di paket Max, Team, dan Enterprise, Opus secara otomatis ditingkatkan ke konteks 1M tanpa konfigurasi tambahan. Ini berlaku untuk kedua kursi Team Standard dan Team Premium. Sonnet 4.6 dengan konteks 1M bukan bagian dari peningkatan otomatis dan memerlukan [penggunaan tambahan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) di setiap paket langganan, termasuk Max.

| Plan                      | Opus dengan konteks 1M                                                                                              | Sonnet 4.6 dengan konteks 1M                                                                                        |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Max, Team, dan Enterprise | Disertakan dengan langganan                                                                                         | Memerlukan [penggunaan tambahan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                       | Memerlukan [penggunaan tambahan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Memerlukan [penggunaan tambahan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API dan pay-as-you-go     | Akses penuh                                                                                                         | Akses penuh                                                                                                         |

Untuk menonaktifkan konteks 1M sepenuhnya, atur `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Ini menghapus varian model 1M dari pemilih model. Lihat [variabel lingkungan](/docs/id/env-vars).

Jendela konteks 1M menggunakan harga model standar tanpa premium untuk token di luar 200K. Untuk paket di mana konteks diperluas disertakan dengan langganan Anda, penggunaan tetap tercakup oleh langganan Anda. Untuk paket yang mengakses konteks diperluas melalui penggunaan tambahan, token ditagihkan ke penggunaan tambahan.

Jika akun Anda mendukung konteks 1M, opsi muncul di pemilih `/model` dalam versi terbaru Claude Code. Jika Anda tidak melihatnya, coba mulai ulang sesi Anda.

Anda juga dapat menggunakan akhiran `[1m]` dengan alias model atau nama model lengkap:

```bash theme={null}
# Gunakan alias opus[1m] atau sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# Atau tambahkan [1m] ke nama model lengkap
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Jendela konteks Sonnet 5
</h4>

Pada Anthropic API, Sonnet 5 selalu berjalan dengan jendela konteks 1M. Tidak ada varian 200K, tidak ada akhiran `[1m]` untuk dipilih, dan tidak ada penggunaan tambahan yang diperlukan di paket apa pun. Sesi melakukan auto-compact sebelum jendela penuh, sekitar 967K token secara default; atur [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/id/env-vars) untuk memilih ambang batas yang berbeda.

Dua konfigurasi menganggarkan jendela pada 200K sebagai gantinya dan melakukan auto-compact pada batas itu:

* **Gateway LLM**: ketika `ANTHROPIC_BASE_URL` mengarah ke [gateway](/docs/id/llm-gateway), Claude Code tidak dapat memverifikasi dukungan 1M. Untuk menggunakan jendela penuh, pilih Sonnet 5 (1M context) di pemilih model, yang dipetakan ke `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: memperlakukan sesi Sonnet 5 sebagai memiliki jendela 200K, untuk penyebaran yang perlu membatasi konteks.

<h2 id="checking-your-current-model">
  Memeriksa model Anda saat ini
</h2>

Anda dapat melihat model mana yang sedang Anda gunakan di dua tempat:

* Dalam [status line](/docs/id/statusline), jika Anda memiliki satu yang dikonfigurasi
* Dalam `/status`, yang juga menampilkan informasi akun Anda

<h2 id="add-a-custom-model-option">
  Tambahkan opsi model kustom
</h2>

Gunakan `ANTHROPIC_CUSTOM_MODEL_OPTION` untuk menambahkan satu entri kustom ke pemilih `/model` tanpa mengganti alias bawaan. Ini berguna untuk pengujian ID model yang tidak tercantum Claude Code secara default. Untuk deployment gateway LLM, Claude Code dapat mengisi pemilih dari endpoint `/v1/models` gateway ketika `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` diatur, jadi variabel ini diperlukan hanya ketika penemuan dinonaktifkan atau tidak mengembalikan model yang Anda inginkan. Lihat [penemuan model gateway](/docs/id/llm-gateway-protocol#model-discovery).

Contoh ini menetapkan ketiga variabel untuk membuat deployment Opus yang dirutekan gateway dapat dipilih:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-4-8"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

Entri kustom muncul di bagian bawah pemilih `/model`. `ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` dan `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` bersifat opsional. Jika dihilangkan, ID model digunakan sebagai nama dan deskripsi default ke `Custom model (<model-id>)`.

Claude Code melewati validasi untuk ID model yang ditetapkan dalam `ANTHROPIC_CUSTOM_MODEL_OPTION`, sehingga Anda dapat menggunakan string apa pun yang diterima endpoint API Anda. Ketika [`availableModels`](#restrict-model-selection) diatur, sertakan ID model kustom dalam daftar izin juga: entri kustom disaring dari pemilih dan pemilihan `--model` darinya ditolak seperti model yang dikecualikan lainnya. ID kustom yang menyematkan nama keluarga, seperti `my-gateway/claude-opus-4-8`, dihitung sebagai entri spesifik untuk keluarga itu dan menonaktifkan wildcard-nya, jadi juga daftarkan versi yang ingin Anda pertahankan dapat dipilih. Lihat [perilaku penggabungan](#merge-behavior).

<h2 id="environment-variables">
  Variabel lingkungan
</h2>

Anda dapat menggunakan variabel lingkungan berikut untuk mengontrol nama model yang dipetakan alias. Setiap nilai harus berupa nama model lengkap, atau pengenal setara untuk penyedia API Anda.

| Variabel lingkungan              | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                             |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Model yang digunakan untuk `fable`, dan ID model yang dikenali Claude Code sebagai Fable 5 untuk [fallback model otomatis](#automatic-model-fallback) pada penyedia pihak ketiga                                                                                                                                                                                                                      |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Model yang digunakan untuk `opus`, atau untuk `opusplan` ketika Plan Mode aktif.                                                                                                                                                                                                                                                                                                                      |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Model yang digunakan untuk `sonnet`, atau untuk `opusplan` ketika Plan Mode tidak aktif.                                                                                                                                                                                                                                                                                                              |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Model yang digunakan untuk `haiku`, atau [fungsionalitas latar belakang](/docs/id/costs#background-token-usage)                                                                                                                                                                                                                                                                                            |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Model yang digunakan untuk semua [subagents](/docs/id/sub-agents#choose-a-model), [agent teams](/docs/id/agent-teams), dan agen yang dijalankan [workflow](/docs/id/workflows). Menerima alias seperti `haiku` atau nama model lengkap, dan mengganti baik parameter `model` per-invocation maupun frontmatter `model` definisi subagent. Atur ke `inherit` untuk menggunakan resolusi model normal sebagai gantinya |

Catatan: `ANTHROPIC_SMALL_FAST_MODEL` sudah usang dan digantikan oleh `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Tetapkan model untuk deployment pihak ketiga
</h3>

Saat menerapkan Claude Code melalui [Amazon Bedrock](/docs/id/amazon-bedrock), [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), [Microsoft Foundry](/docs/id/microsoft-foundry), atau [Claude Platform on AWS](/docs/id/claude-platform-on-aws), tetapkan versi model sebelum meluncurkan ke pengguna.

Tanpa penentapan, Claude Code menggunakan alias model seperti `fable`, `opus`, `sonnet`, dan `haiku` yang diselesaikan ke ID model default bawaan untuk setiap penyedia. Default tersebut dapat tertinggal dari rilis Anthropic terbaru, dan model yang ditunjuknya mungkin belum diaktifkan di akun pengguna. Ketika default tidak tersedia, pengguna Amazon Bedrock dan Google Cloud's Agent Platform melihat pemberitahuan dan sesi kembali ke versi sebelumnya dari model default, atau ke model Sonnet default ketika default adalah model Opus dan tidak ada versi Opus yang tersedia. Pengguna Microsoft Foundry melihat kesalahan sebagai gantinya, karena Microsoft Foundry tidak memiliki pemeriksaan startup yang setara.

<Warning>
  Atur variabel lingkungan model ke ID versi spesifik sebagai bagian dari pengaturan awal Anda. Penentapan memungkinkan Anda mengontrol kapan pengguna Anda pindah ke model baru.
</Warning>

Gunakan variabel lingkungan berikut dengan ID model spesifik versi untuk penyedia Anda:

| Penyedia                      | Contoh                                                               |
| :---------------------------- | :------------------------------------------------------------------- |
| Amazon Bedrock                | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry             | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Terapkan pola yang sama untuk `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, dan `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Untuk ID model saat ini dan warisan di semua penyedia, lihat [Ikhtisar Model](https://platform.claude.com/docs/en/about-claude/models/overview). Untuk meningkatkan pengguna ke versi model baru, perbarui variabel lingkungan ini dan terapkan kembali.

Untuk mengaktifkan [konteks diperluas](#extended-context) untuk model yang ditetapkan, tambahkan `[1m]` ke ID model dalam `ANTHROPIC_DEFAULT_OPUS_MODEL` atau `ANTHROPIC_DEFAULT_SONNET_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Akhiran `[1m]` menerapkan jendela konteks 1M ke semua penggunaan alias `opus` dan `sonnet`, termasuk fase Opus mode-rencana dari [`opusplan`](#opusplan-model-setting).

* Claude Code menghapus akhiran sebelum mengirim ID model ke penyedia Anda.
* Hanya tambahkan `[1m]` ketika model yang mendasar [mendukung konteks 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Akhiran dibaca per variabel, bukan per model. Di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, ID model tanpa `[1m]` dalam satu variabel menggunakan konteks 200K bahkan jika variabel lain menetapkan model yang sama dengan akhiran. Sonnet 5 selalu berjalan dengan jendela 1M pada penyedia ini dan tidak pernah memerlukan akhiran.

<Note>
  Allowlist `availableModels` yang dikirimkan melalui [MDM atau file pengaturan terkelola](/docs/id/settings#settings-files) masih berlaku saat menggunakan penyedia pihak ketiga; [pengaturan yang dikelola server tidak dikirimkan di sana](/docs/id/server-managed-settings#platform-availability). Penyaringan cocok pada alias model seperti `opus`, awalan versi seperti `claude-opus-4-8`, atau ID model lengkap bentuk penyedia. Awalan spesifik penyedia seperti `us.anthropic.` tidak dilepas, jadi untuk memungkinkan model tertentu, daftarkan ID bentuk penyedia yang sama yang ditampilkan pemilih, atau petakan melalui [`modelOverrides`](#override-model-ids-per-version). Akhiran `[1m]` apa pun dilepas dari entri allowlist dan model yang diminta sebelum pencocokan.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Sesuaikan tampilan dan kemampuan model yang ditetapkan
</h3>

Ketika Anda menetapkan model pada penyedia pihak ketiga, ID spesifik penyedia muncul apa adanya di pemilih `/model` dan Claude Code mungkin tidak mengenali fitur mana yang didukung model. Anda dapat mengganti nama tampilan dan mendeklarasikan kemampuan dengan variabel lingkungan pendamping untuk setiap model yang ditetapkan.

Variabel ini berlaku pada penyedia pihak ketiga seperti Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry. Variabel `_NAME` dan `_DESCRIPTION` juga berlaku ketika `ANTHROPIC_BASE_URL` menunjuk ke [gateway LLM](/docs/id/llm-gateway). Mereka tidak berpengaruh saat menghubungkan langsung ke `api.anthropic.com`.

| Variabel lingkungan                                   | Deskripsi                                                                                                                 |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nama tampilan untuk model Opus yang ditetapkan di pemilih `/model`. Default ke ID model saat tidak diatur                 |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Deskripsi tampilan untuk model Opus yang ditetapkan di pemilih `/model`. Default ke `Custom Opus model` saat tidak diatur |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Daftar kemampuan yang dipisahkan koma yang didukung model Opus yang ditetapkan                                            |

Akhiran `_NAME`, `_DESCRIPTION`, dan `_SUPPORTED_CAPABILITIES` yang sama tersedia untuk `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL`, dan `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code mengaktifkan fitur seperti [tingkat usaha](#adjust-effort-level) dan [extended thinking](#extended-thinking) dengan mencocokkan ID model terhadap pola yang dikenal. ID spesifik penyedia seperti ARN Amazon Bedrock atau nama deployment kustom sering kali tidak cocok dengan pola ini, meninggalkan fitur yang didukung dinonaktifkan. Atur `_SUPPORTED_CAPABILITIES` untuk memberi tahu Claude Code fitur mana yang benar-benar didukung model:

| Nilai kemampuan        | Mengaktifkan                                                                                  |
| ---------------------- | --------------------------------------------------------------------------------------------- |
| `effort`               | [Tingkat usaha](#adjust-effort-level) dan perintah `/effort`                                  |
| `xhigh_effort`         | Tingkat usaha `xhigh`                                                                         |
| `max_effort`           | Tingkat usaha `max`                                                                           |
| `thinking`             | [Extended thinking](#extended-thinking)                                                       |
| `adaptive_thinking`    | Penalaran adaptif yang secara dinamis mengalokasikan pemikiran berdasarkan kompleksitas tugas |
| `interleaved_thinking` | Pemikiran antara panggilan alat                                                               |

Ketika `_SUPPORTED_CAPABILITIES` diatur, kemampuan yang tercantum diaktifkan dan kemampuan yang tidak tercantum dinonaktifkan untuk model yang ditetapkan yang cocok. Ketika variabel tidak diatur, Claude Code kembali ke deteksi bawaan berdasarkan ID model.

Contoh ini menetapkan Opus ke ARN model kustom Amazon Bedrock, menetapkan nama yang ramah, dan mendeklarasikan kemampuannya:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Ganti ID model per versi
</h3>

Variabel lingkungan tingkat keluarga di atas mengonfigurasi satu ID model per alias keluarga. Jika Anda perlu memetakan beberapa versi dalam keluarga yang sama ke ID penyedia yang berbeda, gunakan pengaturan `modelOverrides` sebagai gantinya.

`modelOverrides` memetakan ID model Anthropic individual ke string spesifik penyedia yang dikirim Claude Code ke API penyedia Anda. Ketika pengguna memilih model yang dipetakan di pemilih `/model`, Claude Code menggunakan nilai yang Anda konfigurasi alih-alih default bawaan.

Ini memungkinkan administrator enterprise untuk merutekan setiap versi model ke ARN profil inferensi Amazon Bedrock tertentu, nama versi Google Cloud's Agent Platform, atau nama deployment Microsoft Foundry untuk tata kelola, alokasi biaya, atau perutean regional.

Atur `modelOverrides` dalam [file pengaturan](/docs/id/settings#settings-files) Anda:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

Kunci harus berupa ID model Anthropic seperti yang tercantum dalam [Ikhtisar Model](https://platform.claude.com/docs/en/about-claude/models/overview). Untuk ID model bertanggal, sertakan akhiran tanggal persis seperti yang muncul di sana. Kunci yang tidak dikenal diabaikan.

Penggantian menggantikan ID model bawaan yang mendukung setiap entri di pemilih `/model`. Di Amazon Bedrock, penggantian `modelOverrides` mengambil alih profil inferensi apa pun yang ditemukan Claude Code secara otomatis saat startup. Claude Code meneruskan nilai yang sudah bentuk penyedia asli, seperti ARN profil inferensi Amazon Bedrock atau nama deployment Microsoft Foundry, ke penyedia apa adanya.

Penggantian juga berlaku ketika Anda meneruskan ID model Anthropic secara langsung melalui `--model`, variabel lingkungan `ANTHROPIC_MODEL`, atau variabel lingkungan `ANTHROPIC_DEFAULT_*_MODEL`. Di Amazon Bedrock, Google Cloud's Agent Platform, dan [Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint), ID model Anthropic tanpa entri `modelOverrides` diselesaikan ke ID spesifik penyedia yang sama dengan baris pemilih `/model` untuk versi itu, ketika penyedia mendukung versi itu. Mantle mendukung subset versi. Untuk ID model Anthropic di luar subset itu, Claude Code mengirim ID mentah ke Mantle tanpa memetakannya, kecuali entri `modelOverrides` mencakupnya. Sebelum v2.1.200, `--model` dan nilai variabel lingkungan mencapai penyedia apa adanya tanpa melewati peta penggantian.

`modelOverrides` bekerja bersama `availableModels`. Allowlist dievaluasi terhadap ID model Anthropic, bukan nilai penggantian, jadi entri seperti `"opus"` dalam `availableModels` terus cocok bahkan ketika versi Opus dipetakan ke ARN. Ketika `enforceAvailableModels` diatur dalam pengaturan terkelola, Default yang diterapkan diselesaikan melalui `modelOverrides` dari [sumber terkelola dengan prioritas tertinggi](/docs/id/server-managed-settings#settings-precedence) saja. Pemetaan admin, seperti versi yang ditetapkan ke ARN profil inferensi, dihormati dalam Default yang diterapkan. Penggantian dari pengaturan pengguna atau proyek tidak mempengaruhinya.

Ketika `availableModels` diatur dalam [pengaturan terkelola](/docs/id/settings#settings-files), hanya `modelOverrides` dari sumber terkelola itu yang berlaku untuk ID model Anthropic yang diteruskan secara langsung melalui `--model` atau variabel lingkungan di atas. Claude Code mengabaikan penggantian dalam pengaturan pengguna atau proyek untuk ID tersebut, dan tidak pernah menyelesaikan ID yang dikecualikan daftar terkelola melalui `modelOverrides` dari sumber pengaturan apa pun. Pembatasan sumber terkelola ini memerlukan Claude Code v2.1.200 atau lebih baru. Lihat [Batasi pemilihan model](#restrict-model-selection) untuk cara ID yang diblokir ditangani.

<h3 id="prompt-caching-configuration">
  Konfigurasi prompt caching
</h3>

Claude Code secara otomatis menggunakan [prompt caching](/docs/id/prompt-caching) untuk mengoptimalkan kinerja dan mengurangi biaya. Anda dapat menonaktifkan prompt caching secara global atau untuk tingkat model tertentu:

| Variabel lingkungan             | Deskripsi                                                                                             |
| ------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Atur ke `1` untuk menonaktifkan prompt caching untuk semua model. Mengambil alih pengaturan per-model |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Haiku                                |
| `DISABLE_PROMPT_CACHING_SONNET` | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Sonnet                               |
| `DISABLE_PROMPT_CACHING_OPUS`   | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Opus                                 |
| `DISABLE_PROMPT_CACHING_FABLE`  | Atur ke `1` untuk menonaktifkan prompt caching hanya untuk model Fable                                |

Untuk mengubah cache TTL atau mempelajari apa yang memicu cache miss, lihat [Bagaimana Claude Code menggunakan prompt caching](/docs/id/prompt-caching).
