> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Jalankan Claude Code melalui gateway

> Arahkan Claude Code melalui gateway yang di-host sendiri untuk kredensial terpusat, pelacakan penggunaan, dan kontrol biaya. Mencakup arsitektur, gateway aplikasi Claude Anthropic, dan menggunakan produk gateway lainnya.

Gateway adalah proxy yang dijalankan organisasi Anda antara Claude Code dan penyedia model. Claude Code mengirimkan lalu lintas API ke gateway alih-alih langsung ke penyedia, dan gateway meneruskannya menggunakan kredensial yang dipegang organisasi Anda. Pengembang melakukan autentikasi ke gateway daripada memegang kredensial penyedia, sehingga autentikasi, pelacakan penggunaan, anggaran, dan pencatatan audit terjadi di satu tempat yang Anda kontrol.

Claude Code menyertakan gateway yang di-host sendiri, [Claude apps gateway](/docs/id/claude-apps-gateway), dalam biner `claude`, sehingga Anda tidak perlu mengadopsi produk gateway terpisah untuk menjalankan satu. Jika organisasi Anda sudah menjalankan [LLM gateway](/docs/id/llm-gateway), Claude Code juga bekerja dengan itu.

Halaman ini mencakup:

* [Bagaimana gateway berada di antara Claude Code dan penyedia Anda](#how-a-gateway-works)
* [Memilih antara Claude apps gateway dan gateway yang sudah Anda jalankan](#choose-a-gateway)
* [Bagaimana gateway berinteraksi dengan langganan claude.ai](#subscriptions-and-gateways)
* [Apa yang dikonfigurasi secara terpisah dari gateway](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  How a gateway works
</h2>

Setiap Claude Code pengembang ditunjukkan ke alamat gateway dan melakukan autentikasi dengan kredensial yang dikeluarkan gateway.

Gateway melakukan autentikasi pengembang, menerapkan aturan akses dan anggaran apa pun yang Anda konfigurasikan, dan meneruskan permintaan ke penyedia Anda dengan kredensial organisasi. Penyedia dapat berupa API Anthropic atau [penyedia cloud](/docs/id/third-party-integrations) seperti Amazon Bedrock, Agent Platform Google Cloud, atau Microsoft Foundry; konfigurasi gateway memutuskan. Dengan Claude apps gateway, atau gateway lain yang mengekspos titik akhir format Anthropic tunggal, mengubah penyedia tidak memerlukan menyentuh mesin pengembang.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Diagram menunjukkan Claude Code merutekan melalui gateway. Di zona mesin pengembang, CLI Claude Code dan ekstensi VS Code mengirimkan permintaan ke alamat gateway dengan kredensial per-pengembang. Di zona berlabel infrastruktur Anda, gateway menangani autentikasi, pelacakan penggunaan, anggaran, dan perutean, dan meneruskan permintaan dengan kredensial organisasi Anda. Di zona penyedia model, panah solid mengarah ke penyedia yang Anda konfigurasikan, ditampilkan sebagai API Anthropic, dan panah putus-putus mengarah ke opsi penyedia lain, diilustrasikan dengan Amazon Bedrock, Google Cloud, dan Microsoft Foundry sebagai contoh." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

Dua jenis kredensial terlibat:

* **Kredensial pengembang**: setiap pengembang memegang milik mereka sendiri, dikeluarkan oleh gateway. Ini melakukan autentikasi mereka ke gateway dan mengidentifikasi mereka dalam pelacakan penggunaan
* **Kredensial penyedia**: gateway memegang satu kredensial untuk akun penyedia Anda, dibagikan oleh semua lalu lintas yang diteruskan

<h2 id="choose-a-gateway">
  Choose a gateway
</h2>

Claude Code bekerja dengan gateway Anthropic sendiri atau dengan gateway yang sudah dijalankan organisasi Anda.

<h3 id="claude-apps-gateway">
  Claude apps gateway
</h3>

Claude apps gateway adalah gateway yang di-host sendiri milik Anthropic, disertakan dalam biner `claude`. Ini merutekan ke Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry, atau API Anthropic sebagai upstream. Pengembang masuk dengan penyedia identitas perusahaan Anda melalui `/login`, gateway memberlakukan akses model dan [pengaturan terkelola](/docs/id/permissions#managed-settings) berdasarkan grup IdP, dan memancarkan metrik penggunaan [OpenTelemetry Protocol (OTLP)](/docs/id/monitoring-usage) ke tumpukan observabilitas Anda sendiri.

Karena dibangun dan diuji bersama setiap rilis Claude Code, ia meneruskan header dan bidang permintaan yang dikirim Claude Code. Gateway yang dipertahankan secara terpisah memerlukan [aturan penerusan diperbarui](/docs/id/llm-gateway-protocol#forward-as-open-lists) karena header dan bidang tersebut berubah dengan setiap rilis; Claude apps gateway dirilis dengan CLI, jadi tidak ada daftar untuk tetap terkini. Lihat [Ketersediaan dan keterbatasan](/docs/id/claude-apps-gateway#availability-and-limitations) untuk set kecil fitur yang berperilaku berbeda pada sesi gateway.

Masuk gateway adalah langkah SSO browser, dan tidak ada alur token layanan, jadi pipeline CI tanpa pengembang untuk menyetujui masuk tidak dapat melakukan autentikasi melaluinya; konfigurasikan yang terhadap penyedia Anda secara langsung. Sesi SDK Agent dan `claude -p` berjalan pada mesin tempat pengembang telah masuk menggunakan sesi gateway mesin itu dan diatur oleh kebijakannya. Lihat [Pipeline CI dan mesin jarak jauh](/docs/id/claude-apps-gateway#ci-pipelines-and-remote-machines).

Lihat [Claude apps gateway](/docs/id/claude-apps-gateway) untuk menerapkannya.

<h3 id="other-gateways">
  Other gateways
</h3>

Jika organisasi Anda sudah menjalankan gateway LLM atau gateway API, Anda dapat menggunakannya sebagai gantinya. Anthropic tidak mendukung, mempertahankan, atau mengaudit produk gateway lain, dan tidak mendukung perutean Claude Code ke model non-Claude melalui gateway apa pun. Lihat [Other LLM gateways](/docs/id/llm-gateway) untuk daftar periksa peluncuran admin, apa yang harus diimplementasikan gateway, dan cara menunjukkan Claude Code ke sana.

<h2 id="subscriptions-and-gateways">
  Subscriptions and gateways
</h2>

Ketika pengembang terhubung melalui gateway dengan kredensial gateway, penggunaan ditagihkan ke akun penyedia organisasi Anda dengan tarif API, dan langganan claude.ai mereka tidak digunakan atau ditagihkan. Menetapkan [`ANTHROPIC_AUTH_TOKEN`](/docs/id/env-vars) untuk gateway yang Anda jalankan, atau masuk ke Claude apps gateway dengan `/login`, mematikan masuk langganan untuk sesi itu. Setiap permintaan yang diteruskan di bawah kredensial itu ditagihkan ke akun di balik kredensial penyedia gateway.

Pengecualiannya adalah menetapkan hanya `ANTHROPIC_BASE_URL`, tanpa kredensial gateway. Permintaan masih merutekan melalui gateway, tetapi login claude.ai yang disimpan tetap menjadi kredensial aktif, sehingga batas penggunaan dan penagihan langganan berlaku. [Other LLM gateways](/docs/id/llm-gateway#subscriptions-and-gateways) mencakup konfigurasi itu dan apa yang harus diteruskan gateway agar berfungsi.

<h2 id="configure-separately-from-the-gateway">
  Configure separately from the gateway
</h2>

Gateway merutekan permintaan API model. Beberapa hal yang mungkin Anda harapkan untuk ditangani dikonfigurasi di tempat lain:

* **Model mana yang menjawab**: pilih model dengan perintah `/model` atau [variabel lingkungan model](/docs/id/model-config#setting-your-model). Gateway memutuskan ke mana permintaan pergi, bukan model mana yang dipilih pengembang. Claude apps gateway dapat membatasi pilihan dengan daftar izin `availableModels` per-grup, tetapi pengembang masih memilih di dalamnya.
* **Lalu lintas jaringan lainnya**: Claude Code sendiri mengirimkan pemeriksaan versi dan unduhan langsung ke Anthropic, terpisah dari jalur gateway. Apakah aliran telemetri klien opsional juga aktif tergantung pada penyedia Anda; [tabel default telemetri](/docs/id/data-usage#telemetry-services) mencakup setiap kasus. Pada sesi Claude apps gateway yang masuk, kredensial gateway menonaktifkan analitik terikat Anthropic dan, ketika [penerusan telemetri](/docs/id/claude-apps-gateway-config#telemetry) dikonfigurasi, menyematkan ekspor OTLP ke gateway. Jaringan Anda masih memerlukan egress ke [domain yang diperlukan](/docs/id/network-config), atau atur [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars) untuk mematikan aliran opsional.
* **Proxy HTTP perusahaan**: `HTTPS_PROXY` berada di antara Claude Code dan setiap server yang dibicarakannya, termasuk gateway. Jika jaringan Anda memerlukan satu, [konfigurasikan proxy](/docs/id/network-config) selain gateway. Untuk Claude apps gateway yang Anda hosting, [masuk memeriksa bahwa host proxy juga berada di jaringan pribadi](/docs/id/claude-apps-gateway#prerequisites); jika tidak, tambahkan host gateway ke `NO_PROXY` sehingga CLI terhubung langsung ke sana.

<h2 id="next-steps">
  Next steps
</h2>

Halaman berikutnya tergantung pada siapa yang menjalankan gateway. Gateway Anthropic berjalan dari biner `claude` dan memiliki panduan pengaturannya sendiri; gateway yang sudah dijalankan organisasi Anda memiliki protokol untuk diimplementasikan dan daftar periksa peluncuran admin.

* [Claude apps gateway](/docs/id/claude-apps-gateway) untuk menerapkan gateway yang di-host sendiri Anthropic dengan masuk SSO dan telemetri OTLP
* [Other LLM gateways](/docs/id/llm-gateway) untuk apa yang harus diimplementasikan gateway yang sudah dijalankan organisasi Anda, dan cara menunjukkan Claude Code ke sana
* [Set up Claude Code for your organization](/docs/id/admin-setup) untuk keputusan peluncuran yang lebih luas yang merupakan bagian dari gateway
