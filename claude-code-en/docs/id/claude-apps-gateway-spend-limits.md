> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Batas pengeluaran gateway aplikasi Claude

> Batasi pengeluaran setiap pengembang melalui gateway aplikasi Claude berdasarkan hari, minggu, atau bulan. Tetapkan batas dengan Admin API dan gateway memberlakukannya secara langsung pada setiap permintaan.

Batas pengeluaran membatasi berapa banyak yang dapat dikeluarkan setiap pengembang melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway) Anda dalam hari, minggu, atau bulan tertentu. Ketika seorang pengembang melampaui batas mereka, gateway mengembalikan `429` pada permintaan berikutnya dan memblokir mereka sampai periode disetel ulang atau admin menaikkan batas. Gunakan batas pengeluaran untuk memberikan setiap pengembang, grup, atau seluruh organisasi batas maksimal pada kredensial yang dibagikan semua orang.

Gateway aplikasi Claude meneruskan semua inferensi melalui satu kredensial hulu bersama, sehingga tagihan penyedia Anda mengatribusikan semuanya ke kredensial tersebut, bukan ke pengembang individual. Tanpa batas per-pengembang, satu armada agen yang tidak terkontrol dapat menghabiskan seluruh komitmen organisasi. Batas pengeluaran adalah tampilan per-pengembang gateway dan pemutus sirkuit di atas tagihan bersama tersebut.

<h2 id="set-a-cap">
  Tetapkan batas
</h2>

Dengan blok [`admin:`](/docs/id/claude-apps-gateway-config#admin) yang dikonfigurasi dalam `gateway.yaml`, gateway melayani Admin API di `/v1/organizations/spend_limits` dan memberlakukan batas secara langsung pada setiap permintaan inferensi. Batas itu sendiri ditetapkan melalui API tersebut, bukan dalam `gateway.yaml`; setiap permintaan `POST /v1/organizations/spend_limits` membuat atau mengganti satu batas dari `{scope, amount, period}`. API mencerminkan bentuk kawat dari [Admin API](https://platform.claude.com/docs/en/manage-claude/admin-api) publik Anthropic untuk titik akhir batas pengeluaran, sehingga klien HTTP yang ditulis terhadap kontrak tersebut dapat menargetkan gateway dengan mengubah URL dasarnya.

Permintaan ini menetapkan default seluruh organisasi sebesar \$500 per bulan untuk setiap pengembang:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

Permintaan ini menerapkan batas yang lebih ketat sebesar \$100 per hari pada setiap anggota grup `contractors`:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| Bidang       | Nilai                                         | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------ | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`, `rbac_group`, `organization`          | `user` menargetkan satu pengembang berdasarkan OpenID Connect (OIDC) `sub` mereka, ID pengguna stabil yang ditetapkan penyedia identitas Anda; teruskan sebagai `scope.user_id`. `rbac_group` menargetkan [grup IdP](/docs/id/claude-apps-gateway-config#managed) berdasarkan nama; teruskan sebagai `scope.rbac_group_id`. `organization` adalah default seluruh organisasi. Gateway menerima ketiganya; `POST` publik Anthropic hanya untuk pengguna hari ini. |
| `amount`     | String angka keseluruhan sen USD, atau `null` | `null` tidak terbatas. `"0"` adalah batas nol, yang memblokir setiap permintaan.                                                                                                                                                                                                                                                                                                                                                                            |
| `period`     | `daily`, `weekly`, `monthly`                  | Ruang lingkup dapat menahan satu batas per periode, dan masing-masing memberlakukan secara independen: pengembang diblokir jika melampaui salah satu dari mereka.                                                                                                                                                                                                                                                                                           |

Batas grup atau organisasi adalah default per-kursi yang diwarisi setiap anggota, bukan kumpulan bersama. Per periode, batas efektif pengembang diselesaikan dalam urutan ini: penggantian per-pengguna, kemudian yang paling ketat dari batas grup mereka, kemudian default organisasi, kemudian tidak terbatas. [`admin.group_limit_mode: max`](/docs/id/claude-apps-gateway-config#admin) membalik tie-break multi-grup ke yang paling tidak ketat sebagai gantinya.

<h3 id="authenticate-to-the-admin-api">
  Autentikasi ke Admin API
</h3>

Kirim salah satu dari:

* Header `x-api-key` yang cocok dengan kunci dalam [`admin.write_keys`](/docs/id/claude-apps-gateway-config#admin) untuk akses penuh, atau `admin.read_keys` untuk akses `GET`-saja. Setiap kunci membawa `id` yang muncul dalam log audit sebagai `admin-key:<id>`, jadi berikan Terraform, CI, dan setiap otomasi miliknya sendiri.
* Token pembawa gateway yang klaim `groups` mencakup salah satu dari [`admin.admin_groups`](/docs/id/claude-apps-gateway-config#admin). Ini adalah akses penuh dan audit sebagai `oidc:<sub>`, jadi lebih suka untuk admin manusia.

<h2 id="how-enforcement-works">
  Cara penegakan bekerja
</h2>

Pada setiap permintaan `/v1/messages`, gateway menyelesaikan batas pengembang dan pengeluaran periode-ke-tanggal dalam satu kueri Postgres. Jika mereka melampaui batas apa pun, permintaan mengembalikan `429` dengan `error.type: billing_error` dan header `x-should-retry: false`. Pesan adalah `spend limit reached`, diikuti oleh [`admin.blocked_message`](/docs/id/claude-apps-gateway-config#admin) Anda jika ditetapkan.

`/v1/messages/count_tokens` dikecualikan. Penghitungan token gratis, jadi berjalan terlepas dari status batas.

Setelah setiap respons, meter penggunaan membaca jumlah token dari respons saat mengalir ke klien, menghargainya pada harga daftar USD, dan menambah penghitung Postgres untuk ketiga bucket periode. Meter adalah pembaca tunggal pada aliran, jadi byte klien tidak tersentuh dan kegagalan metering tidak merusak respons.

Batas pengeluaran memperkirakan pengeluaran dari jumlah token pada harga daftar USD; mereka adalah pemutus sirkuit, bukan faktur. Untuk penagihan yang berwenang, rekonsiliasi terhadap pelaporan penggunaan penyedia Anda sendiri, seperti Admin API Penggunaan & Biaya Anthropic, log invokasi di Bedrock, atau Cloud Monitoring di Google Cloud.

Penetapan harga menggunakan tabel yang sama yang digunakan Claude Code CLI untuk tampilan biaya sendiri, dengan kanonikalisasi ID model yang sama di seluruh Anthropic, Bedrock (`us.anthropic.…-v1:0`), Agent Platform (`claude-…@date`), dan bentuk ID Foundry. ID model yang tidak dapat ditempatkan tabel, seperti nama penerapan Foundry atau ARN profil inferensi, dihargai pada tingkat default model yang tidak dikenal sebesar \$5/\$25 per juta token input/output daripada nol, jadi ID yang tidak dikenali tidak dapat melewati batas dengan tidak diukur. Gateway memperingatkan saat boot dan sekali per ID saat runtime ketika model dihargai melalui fallback.

Pembatalan klien juga ditagih. Hulu melaporkan token output hanya dalam bingkai terminal aliran, jadi aliran yang dibatalkan tidak membawanya. Meter menyimpan perkiraan lantai konservatif dari ukuran konten yang dialirkan, sekitar empat karakter per token, dan menagihnya ketika dan hanya ketika bingkai penggunaan terminal hilang. Aliran lengkap selalu menagih jumlah yang dilaporkan hulu. Tanpa ini, pengembang yang dibatasi dapat mengalirkan output dan membatalkan setiap permintaan segera sebelum akhir, menghabiskan tanpa pernah dihitung.

<h3 id="postgres-availability">
  Ketersediaan Postgres
</h3>

Kueri pra-pemeriksaan Postgres dengan batas waktu dua detik. Jika toko tidak dapat dijangkau atau waktu habis, penegakan gagal terbuka secara default: permintaan dilanjutkan dan gateway mencatat peringatan. Atur [`enforcement.fail_closed_on_error: true`](/docs/id/claude-apps-gateway-config#enforcement) untuk gagal tertutup sebagai gantinya, yang mengembalikan `429 billing_error` yang sama dengan pesan `spend limit unavailable`. Gagal-terbuka menjaga pemadaman toko agar tidak menjadi pemadaman inferensi; gagal-tertutup menjamin tidak ada pengeluaran yang tidak diukur.

<h2 id="admin-api-reference">
  Referensi Admin API
</h2>

Titik akhir di bawah ini disajikan di bawah `/v1/organizations/spend_limits`.

| Metode dan jalur                               | Deskripsi                                                                             |
| ---------------------------------------------- | ------------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Daftar batas yang dikonfigurasi. Kueri: `?limit=&after_id=&before_id=`.               |
| `POST /v1/organizations/spend_limits`          | Buat atau ganti batas untuk `{scope, period}`.                                        |
| `GET /v1/organizations/spend_limits/{id}`      | Ambil satu batas berdasarkan ID dengan awalan `spl_`.                                 |
| `DELETE /v1/organizations/spend_limits/{id}`   | Hapus satu batas. Mengembalikan `{type: "spend_limit_deleted", id}`.                  |
| `GET /v1/organizations/spend_limits/effective` | Batas yang diselesaikan dan pengeluaran periode-ke-tanggal per prinsipal per periode. |
| `GET /v1/organizations/spend_limits/audit`     | Jejak mutasi admin, terbaru-pertama. Kueri: `?limit=`.                                |

Konvensi mencerminkan Admin API Anthropic:

* `type` pada setiap objek
* ID dengan awalan `spl_`
* Jumlah sebagai string angka keseluruhan sen USD; `POST` menolak `currency` lain apa pun dengan `400`
* Amplop kesalahan `{type: "error", error: {type, message}, request_id}`
* Header respons `request-id` pada setiap respons admin, sukses atau kesalahan, cocok dengan `request_id` badan

Setiap mutasi menulis baris sebelum/sesudah ke `admin_audit` dalam transaksi yang sama, dikaitkan dengan `admin-key:<id>` atau `oidc:<sub>`.

Gateway melayani titik akhir batas pengeluaran saja. Permukaan Admin API lainnya, seperti antrian `spend_limit_increase_requests`, bukan bagian dari admin API gateway.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` mengembalikan skema `SpendSummary` Anthropic: setiap baris adalah prinsipal untuk periode, dengan batas yang diselesaikan, pengeluaran periode-ke-tanggal, dan objek `actor`. Perbedaan khusus gateway:

* `user_id` adalah OIDC `sub`.
* `actor.name` dan `actor.email_address` adalah `null` sampai permintaan inferensi pertama prinsipal melalui gateway. Gateway tidak memiliki direktori pengguna; itu mencatat nilai terakhir terlihat dari JWT sesi setiap pengguna.
* Setiap baris juga membawa array `groups`, grup IdP terakhir terlihat prinsipal. Ini adalah ekstensi gateway sehingga UI admin dapat menunjukkan setiap tingkat batas yang berlaku; klien berbentuk Anthropic mengabaikannya.
* Tanpa filter `user_ids[]`, itu mencantumkan prinsipal dengan pengeluaran yang tercatat, karena gateway tidak dapat menghitung semua anggota organisasi.

Batas bersumber grup diselesaikan terhadap grup terakhir terlihat dengan tie-break `group_limit_mode` yang sama yang digunakan penegakan, jadi penampil menunjukkan batas yang benar-benar berlaku.

| Parameter kueri  | Deskripsi                                                                                                                         |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `user_ids[]`     | Dapat diulang. Filter ke prinsipal spesifik berdasarkan OIDC `sub`.                                                               |
| `period[]`       | Dapat diulang. Filter ke baris `daily`, `weekly`, atau `monthly`.                                                                 |
| `sort`           | `spend_desc` mencantumkan pengguna teratas terlebih dahulu. Memerlukan tepat satu `period[]`.                                     |
| `q`              | Filter substring yang tidak peka huruf besar-kecil atas OIDC `sub`, email terakhir terlihat, dan nama tampilan terakhir terlihat. |
| `limit` / `page` | Ukuran halaman, 1–1000 dengan default 20, dan kursor buram dari `next_page` respons sebelumnya.                                   |

<Warning>
  `q=` dan `user_ids[]=` naik string kueri GET, jadi proxy fronting apa pun atau penyeimbang beban menangkapnya dalam log akses. Jika kebijakan log PII Anda ketat, bersihkan parameter ini di sana.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Mengembalikan jejak mutasi batas pengeluaran: siapa yang mengubah batas mana, snapshot sebelum/sesudah, dan alasan opsional, terbaru-pertama. `has_more` tepat. Titik akhir ini mengikuti konvensi Admin API lokal daripada bentuk kawat pihak pertama.

<h3 id="pagination">
  Paginasi
</h3>

Daftar mentah halaman berdasarkan `after_id` dan `before_id`, yang merupakan ID `spl_…` yang saling eksklusif; hasil diurutkan berdasarkan pembuatan dan `has_more` mencerminkan arah traversal. `/effective` halaman berdasarkan token `next_page` buram yang diteruskan kembali sebagai `?page=`, dengan prinsipal diurutkan naik sehingga halaman tetap stabil saat pengeluaran sedang dicatat. `limit` adalah 1–1000, default 20, di keduanya.

<h2 id="data-lifecycle">
  Siklus hidup data
</h2>

Gateway menyimpan empat tabel terkait pengeluaran; sapuan per jam memberlakukan jendela retensi:

| Tabel              | Isi                                                                                | Retensi                                                                                                      |
| ------------------ | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `spend`            | Penghitung periode-ke-tanggal per-prinsipal dalam sen                              | [`admin.spend_retention_months`](/docs/id/claude-apps-gateway-config#admin), default 13                           |
| `spend_limits`     | Batas yang dikonfigurasi                                                           | Sampai dihapus melalui API                                                                                   |
| `admin_audit`      | Jejak mutasi                                                                       | [`admin.audit_retention_days`](/docs/id/claude-apps-gateway-config#admin), default 365                            |
| `principal_emails` | Email terakhir terlihat setiap prinsipal, nama tampilan, dan grup IdP. Berisi PII. | [`admin.identity_retention_days`](/docs/id/claude-apps-gateway-config#admin) sejak aktivitas terakhir, default 90 |

`identity_retention_days` sengaja lebih pendek dari `spend_retention_months`: identitas yang dihapus provisioning berhenti menyegarkan dan menua, sementara penghitung pengeluaran anonimnya tetap untuk pelaporan tahun-ke-tahun.

Ketika pengembang pergi, hapus batas per-pengguna apa pun melalui `DELETE /v1/organizations/spend_limits/{id}`; pengeluaran dan baris identitas mereka menua pada jendela retensi di atas. Untuk menghapus satu orang segera, untuk offboarding atau permintaan akses subjek data (DSAR), jalankan `DELETE FROM principal_emails WHERE principal = '<sub>'` langsung terhadap database gateway. Itu menghapus satu-satunya tabel yang menyimpan email, nama, dan grup mereka. Baris `spend` dan `admin_audit` mereferensikan OIDC `sub` pseudonim saja dan menua pada jendela mereka sendiri.

<h2 id="related">
  Terkait
</h2>

* [Konfigurasi `admin` dan `enforcement`](/docs/id/claude-apps-gateway-config#admin): mengaktifkan Admin API dan menyetel retensi
* [Panduan penerapan](/docs/id/claude-apps-gateway-deploy#postgres): skema Postgres dan panduan cadangan
