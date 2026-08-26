> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop di WSL

> Jalankan sesi Code di dalam distribusi WSL 2 di Windows

Di Windows, tab Code dapat menjalankan sesi di dalam distribusi WSL 2 alih-alih di Windows itu sendiri. Proses Claude Code sesi, alatnya, dan git semuanya dijalankan di dalam distribusi, menggunakan toolchain Linux-nya dan jalur Linux asli, lingkungan yang sama yang ditargetkan proyek Anda.

Gunakan sesi WSL ketika repositori Anda berada di dalam sistem file distribusi. Bekerja pada file-file tersebut dari Windows melalui sistem file jaringan, yang lambat dan merusak pemantauan file; menjalankan sesi di dalam distribusi menghindari keduanya.

<h2 id="requirements">
  Persyaratan
</h2>

* Windows 10 atau 11 dengan [WSL 2](https://learn.microsoft.com/windows/wsl/install). WSL 1 tidak didukung.
* Setidaknya satu distribusi yang terinstal (misalnya, Ubuntu).
* `git` terinstal di dalam distribusi.

<h2 id="start-a-wsl-session">
  Mulai sesi WSL
</h2>

<Steps>
  <Step title="Pilih distribusi">
    Mulai sesi baru di tab Code dan buka pemilih lingkungan. Distribusi WSL 2 yang terinstal muncul di bagian **WSL**. Pilih satu.
  </Step>

  <Step title="Pilih folder">
    Sesi dimulai di direktori home distribusi. Gunakan pemilih folder untuk memilih folder proyek. Penelusuran terjadi di dalam distribusi, dengan jalur Linux seperti `/home/you/project`.
  </Step>

  <Step title="Percayai folder">
    Sesi pertama di folder menampilkan dialog kepercayaan ruang kerja. Kepercayaan diberikan per distribusi dan folder; mempercayai folder di satu distribusi tidak berlaku untuk distribusi lain atau ke jalur yang sama di Windows.
  </Step>
</Steps>

Sesi pertama di distribusi membutuhkan waktu sedikit lebih lama sementara Claude melakukan pengaturan di dalamnya. Anda juga dapat membuka folder `\\wsl.localhost\...` dari pemilih folder normal, dan itu dibuka kembali di dalam distribusi tersebut.

Folder yang baru-baru ini Anda gunakan muncul di pemilih per distribusi, jadi menghubungkan kembali ke proyek hanya membutuhkan satu klik.

<h2 id="what-works-in-a-wsl-session">
  Apa yang berfungsi dalam sesi WSL
</h2>

Sesi paralel, obrolan samping, tinjauan diff visual, status cabang dan permintaan tarik, dan worktrees semuanya berfungsi, didukung oleh git dan toolchain di dalam distribusi. "Buka di editor" membuka VS Code yang terhubung ke distribusi melalui [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl).

Beberapa fitur belum tersedia di sesi WSL: terminal terintegrasi, konektor dan plugin, forking sesi, panel browser file, dan saran file ketika Anda mengetik `@` di komposer.

<h2 id="managed-devices">
  Perangkat yang dikelola
</h2>

Pada perangkat yang dikelola oleh organisasi, sesi WSL mungkin tidak tersedia. Jika awal sesi gagal dengan pesan bahwa perangkat dikelola, itu dikendalikan oleh administrator Anda. Administrator: lihat [bagaimana pengaturan mencapai perangkat](/docs/id/admin-setup#decide-how-settings-reach-devices) dalam panduan penyebaran.
