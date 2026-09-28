> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Uji lingkungan self-hosted end to end

> Verifikasi gambar runner self-hosted dari CI: dispatch sesi dengan CLI, baca balasan Claude melalui hook Stop, dan skrip loop lengkapnya.

<Note>
  Lingkungan self-hosted berada dalam beta publik pada paket Team dan Enterprise; [Ketersediaan dan batasan](/docs/id/self-hosted-environments#availability-and-limitations) mencakup jalur pengaktifan. Halaman ini adalah resep uji CI; lihat [quickstart](/docs/id/self-hosted-environments-quickstart) untuk setup dan [Deploy ke production](/docs/id/self-hosted-environments-deploy) untuk resep fleet.
</Note>

Dalam [lingkungan self-hosted](/docs/id/self-hosted-environments), Claude Code [cloud sessions](/docs/id/claude-code-on-the-web) berjalan pada gambar runner yang Anda bangun dan pertahankan. Sebelum meluncurkan gambar baru ke lingkungan produksi Anda, jalankan sesi lengkap terhadap lingkungan uji dari skrip: buat sesi, baca balasan Claude, kirim follow-up, dan baca balasan itu juga. Ini adalah bentuk uji smoke CI yang memverifikasi gambar runner Anda, akses git, dan alat kustom apa pun sebelum Anda mempromosikan perubahan.

Resep ini mengasumsikan Anda telah [menyiapkan lingkungan dan runner](/docs/id/self-hosted-environments-quickstart#set-up-an-environment-and-runner), dan bahwa pekerjaan CI Anda memulai proses runner pada host yang sama dengan skrip uji, setup alami untuk menguji gambar runner baru. Hook Stop yang Anda instal pada runner menulis balasan akhir setiap giliran ke file lokal, dan skrip membacanya dari sana, jadi satu-satunya panggilan ke API Anthropic adalah dua dispatch itu sendiri. Jika runner uji Anda berada pada infrastruktur terpisah, lihat [Remote test runners](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Instal hook capture pada runner uji Anda
</h2>

Pembacaan kembali bekerja melalui Claude Code [Stop hook](/docs/id/hooks#stop): ketika Claude menyelesaikan giliran, hook menerima pesan asisten akhir sebagai `last_assistant_message` dalam JSON stdin-nya dan menambahkannya ke `$E2E_REPLY_DIR/<session_id>.txt`. Instal dengan cara yang sama seperti [commit-nudge Stop hook](/docs/id/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), pada `~/.claude/` host runner, yang runner semai ke dalam setiap sesi.

<h3 id="save-the-hook-files">
  Simpan file hook
</h3>

Simpan dua file di bawah pada host runner:

* Blok settings: gabungkan ke `~/.claude/settings.json` pada host runner
* Skrip: simpan sebagai `~/.claude/hooks/e2e-stop-hook-capture.sh` pada host runner dan buat dapat dieksekusi

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook untuk menguji lingkungan self-hosted end to end: menulis balasan asisten akhir setiap
# giliran ke $E2E_REPLY_DIR/<session_id>.txt sehingga driver uji yang berlokasi dapat membacanya tanpa
# memanggil API Anthropic.
# Instal pada runner TEST saja. Memerlukan jq.

# No-op kecuali driver mendengarkan. Jangan pernah gagalkan giliran.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID diekspor dalam bentuk cse_...; session
# id yang dicetak CLI dispatch dalam bentuk session_.... ID yang sama, prefix berbeda.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message tidak ada ketika giliran asisten akhir tidak memiliki
# teks, seperti giliran tool-use-only. Filter `// empty` membuat itu menjadi
# penulisan zero-byte daripada string literal "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  Sebelum Anda memulai runner
</h3>

Hook memiliki persyaratan berikut:

* Instal sebelum Anda memulai runner. Runner mengambil snapshot `~/.claude/` sekali saat startup, jadi hook yang ditambahkan ke runner yang berjalan hanya berlaku setelah restart.
* Ekspor `E2E_REPLY_DIR` ke proses runner. Hook adalah no-op ketika variabel tidak diatur atau direktori tidak ada, jadi atur di mana pun Anda memulai runner, seperti unit systemd, pod spec, atau langkah CI. Skrip uji di bawah juga memerlukan itu.

Instal hook ini hanya pada runner yang melayani lingkungan uji Anda. Ini menulis balasan akhir setiap sesi ke disk kapan pun `E2E_REPLY_DIR` ada, yang tidak berbahaya pada runner CI yang dapat dibuang tetapi bukan sesuatu yang dibawa ke gambar runner lingkungan produksi di mana variabel mungkin diatur secara tidak sengaja.

<h2 id="run-the-test-loop">
  Jalankan loop uji
</h2>

Flag dispatch `--environment` dan `--ref` memerlukan Claude Code v2.1.224 atau lebih baru pada mesin yang menjalankan skrip, lantai yang sama dengan runner itu sendiri. Dengan hook di tempat dan runner dimulai pada host ini, skrip uji:

1. Membuat sesi pada lingkungan uji dengan `claude -p "<prompt>" --environment <environment-id> --output-format json`, dijalankan dari checkout git sehingga CLI dapat auto-detect repositori dari remote `origin`. `--ref <branch>` opsional mendasarkan checkout sesi pada ref bernama daripada HEAD lokal. Perintah membuat sesi, mencetak satu baris JSON yang berisi `session_id`, dan keluar tanpa menunggu balasan Claude.
2. Menunggu balasan muncul di `$E2E_REPLY_DIR/<session_id>.txt`, ditulis oleh hook Stop pada runner setelah giliran selesai.
3. Mengirim follow-up dengan `claude -p "<message>" --cloud <session_id> --output-format json` (lihat [Kirim pesan follow-up ke sesi yang berjalan](/docs/id/claude-code-on-the-web#send-follow-ups-from-the-cli)), yang memposting acara pengguna ke sesi yang ada dan keluar.
4. Menunggu balasan follow-up dengan cara yang sama seperti langkah 2.

<h3 id="environment-dispatch-behavior">
  Perilaku dispatch `--environment`
</h3>

Claude Code membuat sesi, mencetak ID sesi dan tautan ke sesi, dan keluar.

Flag mengambil prioritas atas pengaturan [`remote.defaultEnvironmentId`](/docs/id/settings-reference#remote-defaultenvironmentid). Ini tidak mendukung `--output-format stream-json`, dan tidak dapat digabungkan dengan flag yang melanjutkan, melampirkan, atau prekonfigurasi sesi, seperti `--resume`, `--continue`, `--teleport`, `--session-id`, atau `--init-only`. `--cloud` ditolak dengan ID sesi atau URL, dan dalam run non-interaktif ketika membawa deskripsi. `--cloud` kosong diperlakukan sebagai tidak ada. Dari terminal, Anda dapat meneruskan tugas sebagai deskripsi `--cloud` daripada prompt posisional.

<h2 id="example-script">
  Skrip contoh
</h2>

Skrip di bawah menjalankan loop lengkap terhadap `$CLAUDE_TEST_ENVIRONMENT_ID`, ID `ccpool_...` lingkungan uji Anda, ditampilkan dalam dialog detail lingkungan pada halaman admin atau dikembalikan oleh [panggilan create-environment](#create-a-dedicated-test-environment), dan menegaskan pada frasa sentinel di setiap balasan. Jalankan dari checkout git repositori yang ingin dikerjakan sesi, setelah memulai runner pada host ini dengan hook capture terinstal dan `E2E_REPLY_DIR` diekspor.

```bash theme={null}
#!/usr/bin/env bash
# Uji end-to-end terhadap lingkungan self-hosted, menggunakan pembacaan kembali Stop-hook.
# Prasyarat: `claude auth login` telah dijalankan pada mesin ini (lihat "Authenticate
# from CI" di bawah); jq terinstal; CLAUDE_TEST_ENVIRONMENT_ID menamai
# lingkungan yang runnernya adalah yang di host ini, dengan hook capture
# terinstal dan E2E_REPLY_DIR di lingkungannya.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID adalah ejaan legacy
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Menunggu sampai $E2E_REPLY_DIR/<session_id>.txt berisi $2, atau gagal setelah
# 90 detik. Sesuaikan timeout dengan waktu cold-start lingkungan Anda. File
# ditulis oleh hook Stop pada runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Buat sesi pada lingkungan uji. Jalankan dari checkout git
# sehingga CLI dapat auto-detect repo. --ref menancapkan checkout ke ref bernama
# terlepas dari HEAD lokal.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Tunggu balasan turn-1.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Posting follow-up melalui CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Tunggu balasan turn-2.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

Ganti prompt `TURN1`/`TURN2` dan sentinel `EXPECT1`/`EXPECT2` dengan apa pun yang menjalankan setup Anda, seperti meminta Claude menjalankan salah satu alat MCP kustom Anda dan menegaskan pada outputnya.

<h2 id="remote-test-runners">
  Remote test runners
</h2>

Jika runner uji Anda berada pada infrastruktur terpisah, seperti fleet Kubernetes persisten yang pekerjaan CI Anda tidak dapat berbagi filesystem dengannya, tukar penulisan file dalam hook Stop untuk POST ke endpoint yang driver Anda dengarkan:

```sh theme={null}
#!/bin/sh
# Varian hook capture untuk runner pada infrastruktur terpisah.
# Atur E2E_REPLY_URL pada runner ke endpoint yang driver kontrol.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

Di sisi driver, jalankan apa pun yang menerima POST dan menahan balasan sampai uji memintanya, seperti pendengar HTTP kecil di dalam pekerjaan CI atau penerima webhook yang sudah Anda jalankan. Hook berjalan pada infrastruktur Anda, jadi endpoint hanya perlu dapat dijangkau dari runner Anda.

<h2 id="authenticate-from-ci">
  Autentikasi dari CI
</h2>

Baik `claude -p ... --environment` maupun `claude -p ... --cloud` autentikasi dengan token OAuth claude.ai; kunci API, seperti `sk-ant-xxxxx`, tidak diterima untuk panggilan apa pun. Dua pendekatan membuat token tersedia di CI.

<h3 id="long-lived-ci-host">
  Host CI jangka panjang
</h3>

Jalankan `claude auth login` sekali secara interaktif pada mesin yang menjalankan skrip, menggunakan akun pengguna khusus untuk otomasi. Claude Code menyimpan token di OS keychain pada macOS, atau di `~/.claude/.credentials.json` pada Linux dan Windows. Pada host macOS yang Keychain-nya tidak dapat ditulis, seperti yang khas dalam sesi SSH di mana Keychain login tetap terkunci, Claude Code menyimpan token di `~/.claude/.credentials.json` di sana juga. Lihat [Credential management](/docs/id/authentication#credential-management).

CLI menyegarkan token akses jangka pendek secara otomatis pada setiap invokasi, tetapi hibah refresh-token yang mendasar dibatasi pada 30 hari dari login awal, jadi jalankan kembali `claude auth login` secara interaktif pada host itu setiap 30 hari.

<h3 id="ephemeral-ci-runners">
  Runner CI Ephemeral
</h3>

Tidak ada token CI jangka panjang untuk ini hari ini. Scope yang memberikan kontrol sesi jarak jauh, `user:sessions:claude_code`, dibatasi server-side pada 30 hari, jadi `claude setup-token`, yang mencetak token inference-only satu tahun, tidak mencakupnya. [Environment secret](/docs/id/self-hosted-environments-quickstart#set-up-an-environment-and-runner) juga tidak diterima, karena hanya mengotorisasi runner untuk mendaftar dengan lingkungan, bukan untuk membuat sesi.

Untuk menyediakan login yang disimpan ke runner ephemeral, atur [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` dan `CLAUDE_CODE_OAUTH_SCOPES`](/docs/id/env-vars#variables) sehingga `claude auth login` menukar token tanpa browser; batas 30 hari yang sama berlaku untuk hibah refresh. Hubungi tim akun Anthropic Anda jika Anda memerlukan jalur identitas mesin yang tidak terikat pada akun manusia.

<h2 id="create-a-dedicated-test-environment">
  Buat lingkungan uji khusus
</h2>

Buat dan hapus lingkungan secara terprogram sehingga setiap run CI mendapatkan yang bersih; runner yang pekerjaan CI Anda mulai mendaftar ke lingkungan segar. Panggilan create dan delete di bawah adalah endpoint yang sama yang digunakan halaman admin **Cloud environments** pada claude.ai, dan mereka memerlukan header `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Cetak token admin
</h3>

`$ADMIN_TOKEN` adalah token akses OAuth claude.ai untuk akun yang memegang peran Owner, dicetak dengan cara yang sama seperti [Authenticate from CI](#authenticate-from-ci):

* **Cetak itu**: jalankan `claude auth login` dengan akun yang memegang peran Owner, kemudian baca token akses saat ini dari mana pun [Long-lived CI host](#long-lived-ci-host) mengatakan Claude Code menyimpannya.
* **Baca segar setiap run**: CLI memutar token akses, dan batas hibah refresh 30 hari yang sama berlaku, jadi jangan simpan salinan.
* **Teruskan melalui stdin**: seperti contoh, jadi token tidak pernah mendarat di daftar argumen curl atau log build Anda.

<h3 id="create-the-environment">
  Buat lingkungan
</h3>

Tangkap respons tanpa mencetaknya: `pool_secret` adalah kredensial jangka panjang yang dapat mendaftarkan runner ke lingkungan, jadi simpan sebagai rahasia CI yang disembunyikan dan cetak hanya ID lingkungan. Bentuk `-H @-` yang membuat token keluar dari daftar proses memerlukan curl 7.55 atau lebih baru; curl yang lebih lama memperlakukan `@-` sebagai header literal dan mengirim permintaan tanpa otorisasi.

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

Sampai [Owner mengaktifkan **Allow self-hosted environments**](/docs/id/self-hosted-environments#availability-and-limitations) untuk organisasi, panggilan gagal dengan `403` `permission_error` membaca `self-hosted runners are disabled by your organization's policy`.

Mulai runner pada host ini dengan `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, ditambah hook capture dan `E2E_REPLY_DIR` per [Install the capture hook](#install-the-capture-hook-on-your-test-runner), kemudian jalankan skrip uji.

<h3 id="delete-the-environment">
  Hapus lingkungan
</h3>

Hapus lingkungan ketika run selesai, sehingga setiap run CI dimulai bersih:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
