> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contoh file pengaturan

> File settings.json realistis untuk pengembang, tim, dan organisasi: salin satu, pertahankan kunci yang Anda inginkan, dan ubah nilainya.

Halaman ini berisi tiga contoh file `settings.json`, satu untuk setiap tempat Anda menyimpan pengaturan:

* `settings.json` pengembang di `~/.claude/settings.json`
* `settings.json` tim di `.claude/settings.json`, yang dikomit ke repositori
* `managed-settings.json` organisasi

Masing-masing adalah file yang masuk akal untuk pembaca itu, sehingga Anda dapat melihat bentuknya dan menyalin bagian yang Anda inginkan. Tidak ada satupun yang merupakan baseline yang direkomendasikan. Setiap nilai berasal dari entri kunci di [referensi pengaturan](/docs/id/settings-reference), yang memiliki tipe, default, dan tempat pengaturannya dapat dilakukan.

Setiap contoh memiliki dua tab. **Copyable settings file** adalah file seperti yang Anda simpan. **What each key does** adalah file yang sama dengan komentar di atas setiap kunci; Claude Code tidak menerima komentar dalam file pengaturan, jadi salin dari tab pertama.

<h2 id="your-own-settings">
  Pengaturan Anda sendiri
</h2>

Pengaturan pribadi satu pengembang. Ini memilih model dan upaya, menyesuaikan terminal, dan pra-menyetujui perintah baca-saja dan satu pembacaan file. Semua yang tidak tercantum mempertahankan defaultnya. File seperti ini masuk ke `~/.claude/settings.json`, di mana file ini berlaku untuk setiap proyek yang Anda buka.

<Tabs>
  <Tab title="Copyable settings file">
    Simpan ini sebagai `~/.claude/settings.json`. Ini adalah JSON yang valid tanpa komentar, jadi Anda dapat menempelkannya apa adanya dan menghapus kunci yang tidak Anda inginkan.

    ```json ~/.claude/settings.json theme={null}
    {
      "model": "claude-sonnet-5",
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      "editorMode": "vim",
      "theme": "light-daltonized",
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      "spinnerTipsEnabled": false,
      "preferredNotifChannel": "terminal_bell",
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      "autoUpdatesChannel": "stable",
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>

  <Tab title="What each key does">
    File yang sama dengan komentar di atas setiap kunci. Bacalah di sini; salin dari tab lain, karena Claude Code tidak menerima komentar dalam file pengaturan.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Mulai setiap sesi di Sonnet 5
      "model": "claude-sonnet-5",
      // Jalankan Sonnet 5 di atas tingkat tinggi defaultnya; /effort menyimpan tingkat per model, dan --effort menetapkan satu untuk sesi tunggal
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Pintasan keyboard Vim dalam prompt
      "editorMode": "vim",
      // Tema terang yang ramah buta warna
      "theme": "light-daltonized",
      // Baris status di bawah prompt: nama model dan konteks yang digunakan
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Sembunyikan tips yang berputar di bawah spinner
      "spinnerTipsEnabled": false,
      // Bunyikan bel terminal untuk notifikasi, seperti tugas yang selesai atau prompt izin yang menunggu
      "preferredNotifChannel": "terminal_bell",
      // Biarkan Claude Code menjalankan git diff dan membaca .zshrc Anda tanpa bertanya
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Ambil pembaruan dari saluran stabil
      "autoUpdatesChannel": "stable",
      // Hapus transkrip sesi dan data sesi lokal lainnya yang lebih lama dari 20 hari
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Pengaturan bersama tim
</h2>

Pengaturan bersama satu tim, dikomit ke repositori sehingga semua orang yang mengklonnya mendapatkan izin, hooks, dan marketplace plugin yang sama. Simpan file seperti ini di `.claude/settings.json` di bagian atas repositori. Apa yang perlu diketahui sebelum Anda mengkomitnya:

* **Sesi cloud membacanya juga.** Sesi [cloud session](/docs/id/settings#settings-in-cloud-sessions) dimulai dari klon repositori, jadi file yang dikomit berlaku di sana juga.
* **Telemetri masuk dalam pengaturan terkelola atau pribadi.** Claude Code mengabaikan [variabel pengekspor OpenTelemetry](/docs/id/settings-reference#variables-claude-code-ignores-in-env) dalam file pengaturan repositori, kecuali beberapa nilai yang mematikan telemetri. Atur mereka dalam [pengaturan terkelola](/docs/id/monitoring-usage#administrator-configuration) untuk organisasi Anda, atau dalam `~/.claude/settings.json` setiap orang.
* **Aturan izin menunggu kepercayaan.** Aturan izin dan entri `extraKnownMarketplaces` berlaku setelah setiap orang [mempercayai folder ini sendiri](/docs/id/permissions#project-allow-rules-and-workspace-trust), bukan hanya folder induk; aturan deny dan ask berlaku di setiap sesi, terpercaya atau tidak.
* **Hook adalah skrip di repo.** Hook file ini menjalankan `.claude/hooks/block-rm.sh`; [How a hook resolves](/docs/id/hooks#how-a-hook-resolves) memandu penulisannya.
* **Aturan cocok dengan perintah dan jalur seperti yang ditulis.** `Bash(git push *)` tidak cocok dengan [`git -C . push`](/docs/id/permissions#bash-rule-limits). `Read(./.env)` sendiri menghentikan alat file dan perintah yang menyebutkan file, seperti `cat .env`, tetapi bukan [`grep -r` dijalankan di seluruh direktori](/docs/id/permissions#read-and-edit); blok `sandbox` dalam file ini menutup celah itu, karena sandbox [menambahkan jalur deny `Read` Anda](/docs/id/settings-reference#sandbox-filesystem-denyread) ke apa yang tidak dapat dibaca oleh setiap perintah yang di-sandbox.

<Tabs>
  <Tab title="Copyable settings file">
    Simpan ini sebagai `.claude/settings.json` di bagian atas repositori dan komitnya. Ini adalah JSON yang valid tanpa komentar, jadi Anda dapat menempelkannya apa adanya dan menghapus kunci yang tidak Anda inginkan.

    ```json .claude/settings.json theme={null}
    {
      "permissions": {
        "allow": [
          "Bash(npm run *)"
        ],
        "ask": [
          "Bash(git push *)"
        ],
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      "plansDirectory": "./plans"
    }
    ```
  </Tab>

  <Tab title="What each key does">
    File yang sama dengan komentar di atas setiap kunci. Bacalah di sini; salin dari tab lain, karena Claude Code tidak menerima komentar dalam file pengaturan.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // Jalankan skrip npm tanpa bertanya
        "allow": [
          "Bash(npm run *)"
        ],
        // Konfirmasi sebelum perintah git push
        "ask": [
          "Bash(git push *)"
        ],
        // Tolak pembacaan file env dan folder secrets oleh alat file dan perintah yang membaca file
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Sebelum setiap perintah Bash, jalankan skrip di repo yang dapat membloknya
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      // Daftarkan marketplace plugin tim di setiap klon
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Aktifkan satu plugin dari marketplace itu; plugin dari sumber eksternal seperti repositori GitHub masih memerlukan setiap orang untuk menginstalnya sekali
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Perintah sandbox: direktori build yang dapat ditulis; npm dan example.com pra-diizinkan, host lain masih meminta
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      // Simpan file rencana di dalam repo
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Pengaturan terkelola organisasi
</h2>

File `managed-settings.json` yang menunjukkan bentuk kunci terkelola, dengan satu nilai yang masuk akal untuk masing-masing. Ini bukan kebijakan yang direkomendasikan: pilih kunci yang sesuai dengan persyaratan Anda sendiri dan tetapkan nilai Anda sendiri. Contoh menetapkan kunci-kunci ini:

* `forceLoginMethod` dan `forceLoginOrgUUID` menyematkan metode login dan organisasi
* `availableModels` dan `enforceAvailableModels` membatasi model mana yang dapat digunakan sesi
* `permissions.deny` memblokir dua pembacaan file dan perintah `curl` [seperti yang ditulis Claude](/docs/id/permissions#bash-rule-limits), dan `disableBypassPermissionsMode` menghapus mode izin bypass
* [`allowManagedPermissionRulesOnly`](/docs/id/settings-reference#allowmanagedpermissionrulesonly) dan [`allowManagedMcpServersOnly`](/docs/id/settings-reference#allowmanagedmcpserversonly) membuat daftar izin izin terkelola dan MCP satu-satunya yang berlaku
* `allowedMcpServers` menyematkan server MCP berdasarkan URL
* `strictKnownMarketplaces` memungkinkan satu marketplace plugin
* `sandbox` membuat sandbox perintah dengan daftar izin jaringan tetap dan tanpa retry tanpa sandbox
* `requiredMinimumVersion` menetapkan versi Claude Code minimum
* `cleanupPeriodDays` mempersingkat retensi transkrip sesi dan data lokal lainnya menjadi tujuh hari
* `companyAnnouncements` menampilkan pesan saat startup

Administrator menerapkan file seperti ini sebagai `managed-settings.json`, atau JSON yang sama melalui MDM atau [server-managed settings](/docs/id/server-managed-settings). Satu file yang diterapkan berlaku untuk setiap mesin atau akun yang dicapainya. Untuk memberikan grup nilai yang berbeda, terapkan file atau profil yang berbeda ke grup itu, karena [server-managed settings belum mendukung kebijakan per-grup](/docs/id/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Copyable settings file">
    Terapkan ini sebagai `managed-settings.json`, atau JSON yang sama melalui MDM atau konsol claude.ai. Ini adalah JSON yang valid tanpa komentar; ganti UUID organisasi contoh, URL server, dan marketplace dengan milik Anda sendiri dan hapus kunci yang tidak Anda inginkan.

    ```json managed-settings.json theme={null}
    {
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true,
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      "requiredMinimumVersion": "2.1.150",
      "cleanupPeriodDays": 7,
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>

  <Tab title="What each key does">
    File yang sama dengan komentar di atas setiap kunci. Bacalah di sini; salin dari tab lain, karena Claude Code tidak menerima komentar dalam file pengaturan.

    ```jsonc managed-settings.json theme={null}
    {
      // Hanya login claude.ai, dan hanya di organisasi ini
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Hanya model Opus dan Sonnet; dengan enforceAvailableModels, opsi Default juga mematuhi daftar
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Blokir perintah curl dan pembacaan file .env proyek dan folder secrets di setiap mesin
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Hapus mode bypass-permissions dari setiap sesi
        "disableBypassPermissionsMode": "disable"
      },
      // Abaikan aturan izin dari pengaturan pengguna, proyek, dan lokal
      "allowManagedPermissionRulesOnly": true,
      // Hanya server MCP GitHub, cocok dengan URL daripada dengan nama, karena pengguna dapat
      // memberi nama server apa pun "github". Server yang ditambahkan pengguna yang tidak cocok tidak dimuat, termasuk
      // setiap server stdio ketika daftar hanya memiliki entri URL. Kunci allowManagedMcpServersOnly
      // di bawah membuat daftar terkelola ini satu-satunya daftar izin yang berlaku
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // Plugin hanya dapat berasal dari marketplace ini
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Sandbox setiap perintah yang dijalankan Claude, tolak untuk memulai jika sandbox tidak dapat
      // diatur, dan jangan pernah biarkan perintah yang diblokir mencoba lagi di luar sandbox; jaringan
      // terbatas pada npm dan GitHub, dan pengguna tidak dapat menambahkan domain
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      // Tolak untuk memulai pada versi yang lebih lama dari 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Hapus transkrip sesi dan data sesi lokal lainnya setelah 7 hari
      "cleanupPeriodDays": 7,
      // Pesan yang dilihat setiap pengguna saat startup
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
