> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Luncurkan sesi dari tautan

> Buka sesi terminal Claude Code dari URL. Sematkan tautan `claude-cli://` dalam runbook, peringatan, dan dasbor sehingga klik membuka Claude Code di repo yang tepat dengan prompt yang tepat.

Tautan mendalam adalah URL `claude-cli://` yang membuka Claude Code di jendela terminal baru. URL dapat membawa direktori kerja dan prompt untuk diisi sebelumnya.

Ini memungkinkan Anda berbagi titik awal satu klik untuk tugas: siapa pun dengan Claude Code terinstal yang mengklik tautan akan melihat sesi terbuka dengan prompt sudah diketik. Prompt diisi tetapi tidak dikirim sampai Anda menekan Enter.

Karena tautan mendalam adalah URL, Anda dapat menempatkannya di mana pun tautan dapat ditempatkan:

* Langkah runbook insiden yang membuka repo layanan yang terpengaruh dengan prompt diagnostik
* Peringatan pemantauan atau dasbor yang menautkan ke prompt investigasi untuk metrik tertentu
* Halaman README atau wiki yang membuka proyek dengan prompt onboarding
* Notifikasi kegagalan CI yang mengisi sebelumnya nama pekerjaan yang gagal

Halaman ini mencakup cara [membangun tautan](#build-a-link), [menyematkannya dalam runbook atau memicunya dari shell](#examples), dan [mengelola atau menonaktifkan pendaftaran handler](#registration-and-supported-platforms) di setiap platform.

<h2 id="how-it-works">
  Cara kerjanya
</h2>

Awalan `claude-cli://` adalah skema URL khusus yang Claude Code daftarkan dengan sistem operasi Anda, mirip dengan cara tautan `mailto:` membuka klien email Anda. Tautan dapat berada di halaman web, di wiki, di pesan Slack, atau di aplikasi apa pun yang merender tautan. Ketika Anda mengkliknya:

1. Browser atau aplikasi menyerahkan URL ke sistem operasi Anda.
2. Sistem operasi mengenali awalan `claude-cli://` dan memulai Claude Code di mesin Anda.
3. Jendela terminal baru terbuka dengan Claude Code berjalan di direktori yang ditentukan tautan, dan teks prompt tautan sudah ada di kotak input.
4. Anda membaca prompt, mengeditnya jika mau, dan menekan Enter untuk mengirimnya.

Tautan itu sendiri dapat dihosting di mana saja, tetapi sesi selalu terbuka secara lokal di komputer tempat Anda mengklik. Lihat [Pendaftaran dan platform yang didukung](#registration-and-supported-platforms) untuk emulator terminal mana yang terbuka di setiap sistem operasi.

<Note>
  Platform yang menampilkan tautan harus memungkinkan skema URL khusus. Markdown yang dirender GitHub memungkinkan `http` dan `https` tetapi menghapus skema seperti `claude-cli://` di README, masalah, permintaan tarik, dan wiki. Hanya teks tautan yang ditampilkan, tanpa tautan di belakangnya dan URL tersembunyi. Lihat [Troubleshooting](#the-link-renders-as-plain-text-instead-of-being-clickable) untuk solusi.
</Note>

<h3 id="what-a-launched-session-shows">
  Apa yang ditampilkan sesi yang diluncurkan
</h3>

Tautan mendalam tidak pernah mengeksekusi apa pun dengan sendirinya. Tautan hanya memilih direktori dan mengisi kotak prompt. Jika Anda mengklik tautan dari halaman yang tidak Anda percayai, prompt masih inert: tidak ada yang mencapai model sampai Anda membaca apa yang diisi dan menekan Enter.

Ketika sesi terbuka, spanduk peringatan di bawah kotak input berbunyi `Prompt from an external link` dan tetap terlihat sampai Anda mengirim atau menghapus prompt. Untuk prompt lebih dari 1.000 karakter, peringatan mencakup jumlah karakter dan memberi tahu Anda untuk menggulir dan meninjau teks lengkap sebelum menekan Enter, karena prompt panjang dapat mendorong instruksi keluar layar. Aturan izin, `CLAUDE.md`, dan prompt kepercayaan untuk direktori yang dipilih berlaku dengan cara yang sama seperti untuk sesi lainnya.

<h2 id="build-a-link">
  Bangun tautan
</h2>

Setiap tautan mendalam dimulai dengan `claude-cli://open`, yang merupakan satu-satunya jalur yang diterima handler, diikuti oleh parameter kueri opsional. Bentuk minimal membuka Claude Code di direktori home Anda dengan prompt kosong:

```text theme={null}
claude-cli://open
```

Tambahkan parameter untuk mengontrol di mana sesi dimulai dan apa yang dimuat kotak prompt:

| Parameter | Deskripsi                                                                                                                                                                                                                                                  |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`       | Teks untuk diisi sebelumnya di kotak prompt. [URL-encode](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) nilainya. Gunakan `%0A` untuk jeda baris dalam prompt multi-baris. Maksimal 5.000 karakter. |
| `cwd`     | Jalur absolut untuk digunakan sebagai direktori kerja. Jalur jaringan dan UNC ditolak, dan begitu juga jalur yang berisi karakter kontrol tak terlihat atau bidireksional.                                                                                 |
| `repo`    | Slug `owner/name` GitHub. Claude Code menyelesaikannya ke klon lokal yang telah dilihatnya sebelumnya dan dimulai di sana. Jika Anda tidak memiliki klon yang cocok, sesi terbuka di direktori home Anda.                                                  |

`cwd` dan `repo` adalah [dua cara untuk mengatur direktori kerja](#choose-between-cwd-and-repo). Jika Anda melewatkan keduanya, `cwd` memiliki prioritas dan `repo` diabaikan, bahkan jika jalur `cwd` tidak ada.

Tautan berikut menunjuk ke repositori bernama `acme/payments` dengan prompt diagnostik dua baris. Ganti `acme/payments` dengan slug `owner/name` repositori Anda saat Anda membangun milik Anda sendiri:

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

Mengkliknya membuka jendela terminal baru, memulai Claude Code di klon lokal Anda dari `acme/payments`, dan mengisi kotak prompt dengan teks yang didekode:

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Anda dapat mengedit prompt sebelum menekan Enter untuk mengirimnya. Jika Anda tidak memiliki klon lokal repositori, sesi terbuka di direktori home Anda. Lihat [Pilih antara `cwd` dan `repo`](#choose-between-cwd-and-repo) untuk cara jalur lokal dipilih ketika Anda memiliki beberapa klon atau worktrees.

<h3 id="choose-between-cwd-and-repo">
  Pilih antara `cwd` dan `repo`
</h3>

Gunakan `cwd` ketika semua orang yang mengklik tautan memiliki proyek di jalur absolut yang sama, seperti devcontainer standar atau citra VM.

Gunakan `repo` ketika tautan dibagikan dan setiap orang mengklon ke lokasi yang berbeda. Claude Code menyelesaikan slug ke jalur lokal sebagai berikut:

* Setiap kali Anda menjalankan `claude` di repositori Git, jalur sistem file direktori itu dicatat terhadap slug `owner/name` GitHub repositori.
* Ketika tautan mendalam tiba, `repo` membuka jalur yang cocok apa pun yang Anda gunakan paling baru. Beberapa klon dan worktrees dilacak secara terpisah, jadi ia memilih yang Anda kerjakan terakhir.
* Pencarian hanya menemukan jalur di mana Anda telah menjalankan Claude Code setidaknya sekali.
* Tautan tidak mengubah cabang mana yang diperiksa. Sesi terbuka dalam keadaan apa pun direktori itu saat ini.

Header sambutan menunjukkan jalur mana yang dipilihnya sehingga Anda dapat mengonfirmasi klon yang tepat telah dibuka.

<h2 id="examples">
  Contoh
</h2>

Bagian di bawah menunjukkan dua cara umum untuk menggunakan tautan mendalam: sebagai tautan Markdown dalam dokumen dan sebagai perintah dalam skrip atau alias shell.

<h3 id="embed-a-link-in-a-runbook">
  Sematkan tautan dalam runbook
</h3>

Tautan mendalam dalam runbook memberikan siapa pun yang melakukan triase cara satu klik untuk mulai menyelidiki di repositori yang tepat dengan prompt yang disiapkan. Platform yang merender runbook harus memungkinkan skema URL khusus. Markdown yang dirender GitHub tidak memungkinkan `claude-cli://`, jadi tautan mendalam di README, masalah, atau wiki GitHub hanya menampilkan labelnya tanpa tautan yang dapat diklik. Lihat [catatan troubleshooting](#the-link-renders-as-plain-text-instead-of-being-clickable) untuk solusi.

Prompt adalah bagian dari URL dan harus dikodekan URL. Untuk menghasilkan nilai yang dikodekan, teruskan teks prompt Anda melalui `encodeURIComponent` di konsol browser atau encoder URL apa pun.

Contoh di bawah menambahkan titik masuk investigasi ke runbook insiden untuk layanan bernama `web-gateway`:

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

Untuk menggunakan ini dalam runbook Anda sendiri, ganti `acme/web-gateway` dengan slug repositori layanan Anda. Ini memungkinkan insinyur dengan Claude Code terinstal dan klon lokal repositori itu untuk mengklik langkah 2 dan mulai menyelidiki dengan prompt siap dikirim.

<h3 id="open-a-link-from-the-shell">
  Buka tautan dari shell
</h3>

Anda juga dapat membuka tautan mendalam dari skrip shell, alias, atau otomasi daripada dengan mengkliknya. Panggil perintah pembukaan URL sistem operasi Anda dengan tautan sebagai argumen.

<Tabs>
  <Tab title="macOS">
    Perintah `open` bawaan meneruskan URL ke handler `claude-cli://` yang terdaftar:

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    Sebagian besar lingkungan desktop menyediakan `xdg-open`, yang meneruskan URL ke handler yang terdaftar:

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    Di PowerShell, `Start-Process` meneruskan URL ke handler yang terdaftar:

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    Di `cmd.exe`, `start` memperlakukan argumen pertama yang dikutip sebagai judul jendela, jadi teruskan judul kosong sebelum URL:

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  Pendaftaran dan platform yang didukung
</h2>

Claude Code mendaftarkan handler `claude-cli://` dengan sistem operasi Anda pertama kali Anda memulai sesi interaktif di macOS, Linux, dan Windows. Anda tidak menjalankan perintah install terpisah. Pendaftaran menulis ke lokasi tingkat pengguna saja:

| Platform | Lokasi Handler                                                                                                     |
| -------- | ------------------------------------------------------------------------------------------------------------------ |
| macOS    | `~/Applications/Claude Code URL Handler.app`                                                                       |
| Linux    | `claude-code-url-handler.desktop` di bawah `$XDG_DATA_HOME/applications`, defaultnya `~/.local/share/applications` |
| Windows  | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                                    |

Handler meluncurkan Claude Code di emulator terminal yang terdeteksi. Di macOS, Claude Code mengingat terminal dari sesi interaktif terbaru Anda dan menggunakannya kembali, mendukung iTerm2, Ghostty, kitty, Alacritty, WezTerm, dan Terminal.app. Di Linux, ia menghormati variabel lingkungan `$TERMINAL`, kemudian `x-terminal-emulator`, kemudian daftar emulator umum. Di Windows, ia lebih suka Windows Terminal, kemudian PowerShell, kemudian `cmd.exe`.

Untuk mencegah pendaftaran sepenuhnya, atur [`disableDeepLinkRegistration`](/docs/id/settings) ke `"disable"` di `settings.json`. Untuk memberlakukan ini di seluruh organisasi sehingga pengguna tidak dapat mengaktifkannya kembali, atur di [managed settings](/docs/id/server-managed-settings) sebagai gantinya.

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  Buka tab VS Code alih-alih terminal
</h2>

Ekstensi VS Code mendaftarkan handler sendiri di `vscode://anthropic.claude-code/open`, yang membuka tab editor Claude Code daripada jendela terminal. Lihat [Luncurkan tab VS Code dari alat lain](/docs/id/vs-code#launch-a-vs-code-tab-from-other-tools) untuk parameter URL itu.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="clicking-the-link-does-nothing">
  Mengklik tautan tidak melakukan apa pun
</h3>

Handler mungkin belum terdaftar. Mulai sesi `claude` interaktif sekali di mesin itu, keluar, dan coba tautan lagi. Jika Anda berada di Linux tanpa lingkungan desktop, `xdg-open` mungkin tidak memiliki apa pun untuk dikirim.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  Tautan dirender sebagai teks biasa alih-alih dapat diklik
</h3>

Beberapa renderer Markdown hanya memungkinkan tautan `http` dan `https` dan menghapus skema URL lainnya. GitHub melakukan ini di README, masalah, permintaan tarik, dan wiki: `[label](claude-cli://...)` dirender sebagai hanya `label`, tanpa tautan dan URL dihapus. Di platform ini, letakkan tautan mendalam dalam blok kode sehingga pembaca dapat melihat URL dan menempel ke bilah alamat browser mereka.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  Sesi terbuka di direktori home saya alih-alih repo
</h3>

Parameter `repo` hanya menyelesaikan ke klon yang telah dilihat Claude Code. Jalankan `claude` di dalam klon sekali sehingga jalurnya dicatat, atau alihkan tautan untuk menggunakan `cwd` dengan jalur absolut.

<h3 id="the-link-opens-the-wrong-terminal">
  Tautan membuka terminal yang salah
</h3>

Di macOS, mulai `claude` di terminal pilihan Anda sekali dan tautan mendalam berikutnya akan menggunakannya. Di Linux, atur variabel lingkungan `$TERMINAL` ke nama perintah emulator pilihan Anda. Di Windows, urutannya tetap: instal Windows Terminal jika Anda ingin tautan terbuka di sana alih-alih jendela PowerShell atau `cmd.exe`.

<h2 id="learn-more">
  Pelajari lebih lanjut
</h2>

Halaman ini mencakup cara terkait untuk meluncurkan atau memperluas sesi Claude Code:

* [Skills](/docs/id/skills): simpan prompt runbook panjang sebagai `/skill` di repo sehingga parameter `q` tautan mendalam hanya perlu menamainya
* [Non-interactive mode](/docs/id/headless): jalankan Claude dari skrip dan tangkap output tanpa membuka terminal
