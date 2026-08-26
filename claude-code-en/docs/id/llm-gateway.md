> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gateway LLM lainnya

> Arahkan Claude Code melalui gateway LLM yang sudah dijalankan organisasi Anda. Mencakup menghubungkan Claude Code ke gateway, meluncurkannya untuk organisasi Anda, dan apa yang Claude Code kirimkan ke gateway.

Bagian ini mencakup penggunaan produk gateway yang sudah dijalankan organisasi Anda, bukan [gateway aplikasi Claude](/docs/id/claude-apps-gateway). Untuk apa itu gateway, bagaimana gateway berada di antara Claude Code dan penyedia Anda, dan cara memilih antara gateway aplikasi Claude dan produk lain, lihat [gambaran umum gateway](/docs/id/gateways).

<Note>
  * Jika Anda adalah pengembang yang terhubung ke gateway yang ada: [hubungkan Claude Code ke gateway Anda](/docs/id/llm-gateway-connect)
  * Jika Anda adalah admin yang meluncurkan gateway untuk organisasi Anda: [terapkan dan distribusikan gateway](/docs/id/llm-gateway-rollout)
  * Jika Anda mengonfigurasi produk gateway: [referensi protokol gateway](/docs/id/llm-gateway-protocol)
</Note>

Setiap gateway yang mengekspos [format API yang didukung](/docs/id/llm-gateway-protocol#api-formats) berfungsi. Anthropic tidak mendukung, memelihara, atau mengaudit produk gateway pihak ketiga, dan tidak mendukung perutean Claude Code ke model non-Claude melalui gateway apa pun. Terapkan gateway mengikuti dokumentasinya sendiri, kemudian selesaikan sisi Claude Code dengan [langkah-langkah peluncuran di bawah](#roll-out-a-gateway).

<h2 id="what-a-gateway-provides">
  Apa yang disediakan gateway
</h2>

Gateway memberikan organisasi Anda satu tempat untuk mengelola:

* **Kredensial**: kunci penyedia tetap di sisi server; pengembang memegang kredensial gateway sebagai gantinya
* **Pelacakan penggunaan**: atribusikan penggunaan berdasarkan pengembang atau tim, terlepas dari penyedia mana yang melayani permintaan
* **Kontrol biaya**: terapkan anggaran dan batas laju di satu tempat
* **Pencatatan audit**: catat setiap permintaan model untuk kepatuhan
* **Peralihan penyedia**: ubah penyedia dalam konfigurasi gateway, tanpa menyentuh mesin pengembang

Semua ini kecuali peralihan penyedia berlaku apakah upstream adalah API Anthropic atau [penyedia cloud](/docs/id/third-party-integrations). Peralihan penyedia tanpa mengonfigurasi ulang mesin pengembang juga bergantung pada gateway yang mengekspos satu [endpoint format Anthropic](/docs/id/llm-gateway-protocol#api-formats) terlepas dari upstream; gateway yang mengekspos format penyedia sendiri mengikat konfigurasi klien ke penyedia itu.

Pertukaran adalah bahwa gateway menjadi infrastruktur yang dioperasikan organisasi Anda. Claude Code menambahkan kemampuan dengan setiap rilis, dan gateway yang tidak meneruskannya merusak fitur yang sesuai, jadi produk gateway perlu tetap diperbarui seiring Claude Code berkembang. [Referensi protokol gateway](/docs/id/llm-gateway-protocol) mencakup apa yang harus diteruskan.

<h2 id="roll-out-a-gateway">
  Luncurkan gateway
</h2>

Ketika Anda siap meluncurkan gateway LLM ke organisasi Anda, urutannya sama terlepas dari produk gateway mana yang Anda pilih:

1. Terapkan gateway dan berikan kredensial penyedia Anda, sehingga dapat mengautentikasi permintaan yang diteruskannya.
2. Keluarkan setiap pengembang kredensial gateway, sehingga penggunaan dikaitkan dengan pengembang dan offboarding mencabut satu kredensial.
3. Distribusikan konfigurasi melalui [file pengaturan terkelola](/docs/id/settings#settings-files) dan alat rahasia Anda, sehingga setiap mesin menerima URL dasar dan kredensial. Ketika keduanya didistribusikan, pengembang tidak mengonfigurasi apa pun. Jika Anda tidak memiliki distribusi pengaturan, pengembang mengikuti [halaman koneksi](/docs/id/llm-gateway-connect) untuk menetapkan variabel sendiri.
4. Minta setiap pengembang [memeriksa konfigurasi di Claude Code](/docs/id/llm-gateway-connect#check-for-an-existing-configuration), sehingga masalah distribusi muncul sebelum mereka bergantung pada gateway.

[Luncurkan gateway LLM untuk organisasi Anda](/docs/id/llm-gateway-rollout) menjelaskan setiap langkah dan menunjukkan file konfigurasi untuk didistribusikan di setiap langkah. Gateway adalah satu bagian dari pengaturan organisasi; untuk penegakan kebijakan, visibilitas penggunaan, dan keputusan penanganan data, lihat [Siapkan Claude Code untuk organisasi Anda](/docs/id/admin-setup).

<h2 id="subscriptions-and-gateways">
  Langganan dan gateway
</h2>

Sementara [variabel kredensial gateway](/docs/id/llm-gateway-connect#set-the-credential-variable) atau `apiKeyHelper` aktif, langganan claude.ai pengembang tidak digunakan: kredensial menggantikan login langganan untuk sesi itu, dan batas penggunaan langganan tidak berlaku. Lalu lintas itu ditagih per token kepada siapa pun yang memiliki kredensial yang diteruskan gateway, seperti akun Anthropic Console organisasi Anda, atau akun Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry Anda ketika gateway merutekan ke sana.

[`ANTHROPIC_BASE_URL`](/docs/id/llm-gateway-connect#set-the-base-url-and-credential) adalah variabel yang menunjukkan Claude Code ke gateway. Menetapkan hanya variabel itu, tanpa kredensial gateway, tidak menggantikan langganan. Permintaan masih merutekan melalui gateway, tetapi login claude.ai yang disimpan tetap menjadi kredensial aktif, sehingga batas penggunaan dan penagihan berlaku. Gateway yang meneruskan lalu lintas ini ke Anthropic harus meneruskan kemampuan OAuth di `anthropic-beta`; lihat [referensi header permintaan](/docs/id/llm-gateway-protocol#request-headers).

<h2 id="related-pages">
  Halaman terkait
</h2>

* [Gambaran umum gateway](/docs/id/gateways): bagaimana gateway bekerja dan cara memilih antara gateway aplikasi Claude dan produk lain
* [Gateway aplikasi Claude](/docs/id/claude-apps-gateway): gateway yang dihosting sendiri oleh Anthropic dengan masuk SSO dan telemetri OTLP
* [Hubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect): atur URL dasar dan kredensial di mesin Anda sendiri, dengan konfigurasi per-permukaan dan tabel pemecahan masalah
* [Luncurkan gateway LLM untuk organisasi Anda](/docs/id/llm-gateway-rollout): daftar periksa admin untuk menerapkan gateway, mengeluarkan kredensial pengembang, dan mendistribusikan pengaturan terkelola
* [Referensi protokol gateway](/docs/id/llm-gateway-protocol): apa yang Claude Code kirimkan ke gateway, untuk operator yang mengonfigurasi satu, mencakup endpoint, header untuk diteruskan, dan pass-through fitur
