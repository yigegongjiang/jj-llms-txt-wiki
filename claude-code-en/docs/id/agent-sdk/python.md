> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi Agent SDK - Python

> Referensi API lengkap untuk Python Agent SDK, termasuk semua fungsi, tipe, dan kelas.

<h2 id="installation">
  Instalasi
</h2>

Instal paket ke dalam lingkungan virtual. Pada instalasi Python Debian, Ubuntu, dan Homebrew terbaru, menjalankan `pip install` terhadap Python sistem gagal dengan `error: externally-managed-environment`.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Untuk uv, Windows PowerShell, dan pengaturan kunci API, lihat [Memulai dalam gambaran umum Agent SDK](/docs/id/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Memilih antara `query()` dan `ClaudeSDKClient`
</h2>

Python SDK menyediakan dua cara untuk berinteraksi dengan Claude Code:

<h3 id="quick-comparison">
  Perbandingan cepat
</h3>

| Fitur               | `query()`                                            | `ClaudeSDKClient`                           |
| :------------------ | :--------------------------------------------------- | :------------------------------------------ |
| **Sesi**            | Membuat sesi baru secara default                     | Menggunakan kembali sesi yang sama          |
| **Percakapan**      | Pertukaran tunggal                                   | Beberapa pertukaran dalam konteks yang sama |
| **Koneksi**         | Dikelola secara otomatis                             | Kontrol manual                              |
| **Streaming Input** | ✅ Didukung                                           | ✅ Didukung                                  |
| **Interrupts**      | ❌ Tidak didukung                                     | ✅ Didukung                                  |
| **Hooks**           | ✅ Didukung                                           | ✅ Didukung                                  |
| **Custom Tools**    | ✅ Didukung                                           | ✅ Didukung                                  |
| **Continue Chat**   | Manual melalui `continue_conversation` atau `resume` | ✅ Otomatis                                  |
| **Use Case**        | Tugas sekali jalan                                   | Percakapan berkelanjutan                    |

<h3 id="when-to-use-query-one-off-tasks">
  Kapan menggunakan `query()` (tugas sekali jalan)
</h3>

**Terbaik untuk:**

* Pertanyaan sekali jalan di mana Anda tidak memerlukan riwayat percakapan
* Tugas independen yang tidak memerlukan konteks dari pertukaran sebelumnya
* Skrip otomasi sederhana
* Ketika Anda menginginkan awal yang segar setiap kali

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Kapan menggunakan `ClaudeSDKClient` (percakapan berkelanjutan)
</h3>

**Terbaik untuk:**

* **Melanjutkan percakapan** - Ketika Anda memerlukan Claude untuk mengingat konteks
* **Pertanyaan lanjutan** - Membangun berdasarkan respons sebelumnya
* **Aplikasi interaktif** - Antarmuka obrolan, REPL
* **Logika berbasis respons** - Ketika tindakan berikutnya bergantung pada respons Claude
* **Kontrol sesi** - Mengelola siklus hidup percakapan secara eksplisit

<h2 id="functions">
  Fungsi
</h2>

<h3 id="query">
  `query()`
</h3>

Membuat sesi baru untuk setiap interaksi dengan Claude Code secara default. Mengembalikan async iterator yang menghasilkan pesan saat tiba. Setiap panggilan ke `query()` dimulai segar tanpa memori interaksi sebelumnya kecuali Anda melewatkan `continue_conversation=True` atau `resume` dalam [`ClaudeAgentOptions`](#claudeagentoptions). Lihat [Sessions](/docs/id/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Parameter
</h4>

| Parameter   | Tipe                         | Deskripsi                                                                |
| :---------- | :--------------------------- | :----------------------------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | Prompt input sebagai string atau async iterable untuk mode streaming     |
| `options`   | `ClaudeAgentOptions \| None` | Objek konfigurasi opsional (default ke `ClaudeAgentOptions()` jika None) |
| `transport` | `Transport \| None`          | Transport kustom opsional untuk berkomunikasi dengan proses CLI          |

<h4 id="returns">
  Pengembalian
</h4>

Mengembalikan `AsyncIterator[Message]` yang menghasilkan pesan dari percakapan.

<h4 id="example-with-options">
  Contoh - Dengan opsi
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(prompt="Create a Python web server", options=options):
        print(message)


asyncio.run(main())
```

<h3 id="tool">
  `tool()`
</h3>

Dekorator untuk mendefinisikan tools MCP dengan keamanan tipe.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Parameter
</h4>

| Parameter      | Tipe                                            | Deskripsi                                                                |
| :------------- | :---------------------------------------------- | :----------------------------------------------------------------------- |
| `name`         | `str`                                           | Pengenal unik untuk tool                                                 |
| `description`  | `str`                                           | Deskripsi yang dapat dibaca manusia tentang apa yang dilakukan tool      |
| `input_schema` | `type \| dict[str, Any]`                        | Skema yang mendefinisikan parameter input tool (lihat di bawah)          |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotasi tool MCP opsional yang memberikan petunjuk perilaku kepada klien |

<h4 id="input-schema-options">
  Opsi skema input
</h4>

1. **Pemetaan tipe sederhana** (direkomendasikan):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Format JSON Schema** (untuk validasi kompleks):
   ```python theme={null}
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0},
       },
       "required": ["text"],
   }
   ```

<h4 id="returns-2">
  Pengembalian
</h4>

Fungsi dekorator yang membungkus implementasi tool dan mengembalikan instance `SdkMcpTool`.

<h4 id="example">
  Contoh
</h4>

```python theme={null}
from claude_agent_sdk import tool
from typing import Any


@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Hello, {args['name']}!"}]}
```

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Diimpor ulang dari `mcp.types` (juga tersedia sebagai `from claude_agent_sdk import ToolAnnotations`). Semua field adalah petunjuk opsional; klien tidak boleh mengandalkannya untuk keputusan keamanan.

| Field             | Tipe           | Default | Deskripsi                                                                                                                                    |
| :---------------- | :------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`  | Judul yang dapat dibaca manusia untuk tool                                                                                                   |
| `readOnlyHint`    | `bool \| None` | `False` | Jika `True`, tool tidak memodifikasi lingkungannya                                                                                           |
| `destructiveHint` | `bool \| None` | `True`  | Jika `True`, tool dapat melakukan pembaruan destruktif (hanya bermakna ketika `readOnlyHint` adalah `False`)                                 |
| `idempotentHint`  | `bool \| None` | `False` | Jika `True`, panggilan berulang dengan argumen yang sama tidak memiliki efek tambahan (hanya bermakna ketika `readOnlyHint` adalah `False`)  |
| `openWorldHint`   | `bool \| None` | `True`  | Jika `True`, tool berinteraksi dengan entitas eksternal (misalnya, pencarian web). Jika `False`, domain tool ditutup (misalnya, tool memori) |

```python theme={null}
from claude_agent_sdk import tool, ToolAnnotations
from typing import Any


@tool(
    "search",
    "Search the web",
    {"query": str},
    annotations=ToolAnnotations(readOnlyHint=True, openWorldHint=True),
)
async def search(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Results for: {args['query']}"}]}
```

<h3 id="create_sdk_mcp_server">
  `create_sdk_mcp_server()`
</h3>

Buat server MCP dalam proses yang berjalan dalam aplikasi Python Anda.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Parameter
</h4>

| Parameter | Tipe                            | Default   | Deskripsi                                               |
| :-------- | :------------------------------ | :-------- | :------------------------------------------------------ |
| `name`    | `str`                           | -         | Pengenal unik untuk server                              |
| `version` | `str`                           | `"1.0.0"` | String versi server                                     |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`    | Daftar fungsi tool yang dibuat dengan dekorator `@tool` |

<h4 id="returns-3">
  Pengembalian
</h4>

Mengembalikan objek `McpSdkServerConfig` yang dapat diteruskan ke `ClaudeAgentOptions.mcp_servers`.

<h4 id="example-2">
  Contoh
</h4>

```python theme={null}
from claude_agent_sdk import tool, create_sdk_mcp_server


@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {"content": [{"type": "text", "text": f"Sum: {args['a'] + args['b']}"}]}


@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {"content": [{"type": "text", "text": f"Product: {args['a'] * args['b']}"}]}


calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply],  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"],
)
```

<h3 id="list_sessions">
  `list_sessions()`
</h3>

Mencantumkan sesi masa lalu dengan metadata. Filter berdasarkan direktori proyek atau cantumkan sesi di semua proyek. Sinkron; mengembalikan segera.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Parameter
</h4>

| Parameter           | Tipe          | Default | Deskripsi                                                                                            |
| :------------------ | :------------ | :------ | :--------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None`  | Direktori untuk mencantumkan sesi. Ketika dihilangkan, mengembalikan sesi di semua proyek            |
| `limit`             | `int \| None` | `None`  | Jumlah maksimal sesi yang akan dikembalikan                                                          |
| `offset`            | `int`         | `0`     | Jumlah sesi yang akan dilewati dari awal hasil yang diurutkan. Gunakan dengan `limit` untuk paginasi |
| `include_worktrees` | `bool`        | `True`  | Ketika `directory` berada di dalam repositori git, sertakan sesi dari semua jalur worktree           |

<h4 id="return-type-sdksessioninfo">
  Tipe pengembalian: `SDKSessionInfo`
</h4>

| Properti        | Tipe          | Deskripsi                                                                             |
| :-------------- | :------------ | :------------------------------------------------------------------------------------ |
| `session_id`    | `str`         | Pengenal sesi unik                                                                    |
| `summary`       | `str`         | Judul tampilan: judul kustom, ringkasan yang dihasilkan otomatis, atau prompt pertama |
| `last_modified` | `int`         | Waktu modifikasi terakhir dalam milidetik sejak epoch                                 |
| `file_size`     | `int \| None` | Ukuran file sesi dalam byte (`None` untuk backend penyimpanan jarak jauh)             |
| `custom_title`  | `str \| None` | Judul sesi yang ditetapkan pengguna                                                   |
| `first_prompt`  | `str \| None` | Prompt pengguna bermakna pertama dalam sesi                                           |
| `git_branch`    | `str \| None` | Cabang Git di akhir sesi                                                              |
| `cwd`           | `str \| None` | Direktori kerja untuk sesi                                                            |
| `tag`           | `str \| None` | Tag sesi yang ditetapkan pengguna (lihat [`tag_session()`](#tag_session))             |
| `created_at`    | `int \| None` | Waktu pembuatan sesi dalam milidetik sejak epoch                                      |

<h4 id="example-3">
  Contoh
</h4>

Cetak 10 sesi terbaru untuk proyek. Hasil diurutkan berdasarkan `last_modified` menurun, jadi item pertama adalah yang terbaru. Hilangkan `directory` untuk mencari di semua proyek.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Mengambil pesan dari sesi masa lalu. Sinkron; mengembalikan segera.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Parameter
</h4>

| Parameter    | Tipe          | Default    | Deskripsi                                                                   |
| :----------- | :------------ | :--------- | :-------------------------------------------------------------------------- |
| `session_id` | `str`         | diperlukan | ID sesi untuk mengambil pesan                                               |
| `directory`  | `str \| None` | `None`     | Direktori proyek untuk dilihat. Ketika dihilangkan, mencari di semua proyek |
| `limit`      | `int \| None` | `None`     | Jumlah maksimal pesan yang akan dikembalikan                                |
| `offset`     | `int`         | `0`        | Jumlah pesan yang akan dilewati dari awal                                   |

<h4 id="return-type-sessionmessage">
  Tipe pengembalian: `SessionMessage`
</h4>

| Properti             | Tipe                           | Deskripsi                                  |
| :------------------- | :----------------------------- | :----------------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Peran pesan                                |
| `uuid`               | `str`                          | Pengenal pesan unik                        |
| `session_id`         | `str`                          | Pengenal sesi                              |
| `message`            | `Any`                          | Konten pesan mentah                        |
| `parent_tool_use_id` | `None`                         | Dicadangkan untuk penggunaan di masa depan |

<h4 id="example-4">
  Contoh
</h4>

```python theme={null}
from claude_agent_sdk import list_sessions, get_session_messages

sessions = list_sessions(limit=1)
if sessions:
    messages = get_session_messages(sessions[0].session_id)
    for msg in messages:
        print(f"[{msg.type}] {msg.uuid}")
```

<h3 id="get_session_info">
  `get_session_info()`
</h3>

Membaca metadata untuk sesi tunggal berdasarkan ID tanpa memindai direktori proyek lengkap. Sinkron; mengembalikan segera.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Parameter
</h4>

| Parameter    | Tipe          | Default    | Deskripsi                                                                     |
| :----------- | :------------ | :--------- | :---------------------------------------------------------------------------- |
| `session_id` | `str`         | diperlukan | UUID sesi untuk dicari                                                        |
| `directory`  | `str \| None` | `None`     | Jalur direktori proyek. Ketika dihilangkan, mencari di semua direktori proyek |

Mengembalikan [`SDKSessionInfo`](#return-type-sdksessioninfo), atau `None` jika sesi tidak ditemukan.

<h4 id="example-5">
  Contoh
</h4>

Cari metadata sesi tunggal tanpa memindai direktori proyek. Berguna ketika Anda sudah memiliki ID sesi dari run sebelumnya.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Mengganti nama sesi dengan menambahkan entri judul kustom. Panggilan berulang aman; judul terbaru menang. Sinkron.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Parameter
</h4>

| Parameter    | Tipe          | Default    | Deskripsi                                                                     |
| :----------- | :------------ | :--------- | :---------------------------------------------------------------------------- |
| `session_id` | `str`         | diperlukan | UUID sesi untuk diganti nama                                                  |
| `title`      | `str`         | diperlukan | Judul baru. Harus tidak kosong setelah menghapus spasi putih                  |
| `directory`  | `str \| None` | `None`     | Jalur direktori proyek. Ketika dihilangkan, mencari di semua direktori proyek |

Menimbulkan `ValueError` jika `session_id` bukan UUID yang valid atau `title` kosong; `FileNotFoundError` jika sesi tidak dapat ditemukan.

<h4 id="example-6">
  Contoh
</h4>

Ganti nama sesi terbaru sehingga lebih mudah ditemukan nanti. Judul baru muncul di [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) pada pembacaan berikutnya.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Menandai sesi. Teruskan `None` untuk menghapus tag. Panggilan berulang aman; tag terbaru menang. Sinkron.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Parameter
</h4>

| Parameter    | Tipe          | Default    | Deskripsi                                                                     |
| :----------- | :------------ | :--------- | :---------------------------------------------------------------------------- |
| `session_id` | `str`         | diperlukan | UUID sesi untuk ditandai                                                      |
| `tag`        | `str \| None` | diperlukan | String tag, atau `None` untuk menghapus. Disanitasi Unicode sebelum disimpan  |
| `directory`  | `str \| None` | `None`     | Jalur direktori proyek. Ketika dihilangkan, mencari di semua direktori proyek |

Menimbulkan `ValueError` jika `session_id` bukan UUID yang valid atau `tag` kosong setelah sanitasi; `FileNotFoundError` jika sesi tidak dapat ditemukan.

<h4 id="example-7">
  Contoh
</h4>

Tandai sesi, kemudian filter berdasarkan tag itu pada pembacaan nanti. Teruskan `None` untuk menghapus tag yang ada.

```python theme={null}
from claude_agent_sdk import list_sessions, tag_session

# Tag a session
tag_session("550e8400-e29b-41d4-a716-446655440000", "needs-review")

# Later: find all sessions with that tag
for session in list_sessions(directory="/path/to/project"):
    if session.tag == "needs-review":
        print(session.summary)
```

<h2 id="classes">
  Kelas
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Mempertahankan sesi percakapan di beberapa pertukaran.** Ini adalah setara Python dari cara fungsi `query()` SDK TypeScript bekerja secara internal - ia membuat objek klien yang dapat melanjutkan percakapan.

<h4 id="key-features">
  Fitur Utama
</h4>

* **Kontinuitas sesi**: Mempertahankan konteks percakapan di beberapa panggilan `query()`
* **Percakapan yang sama**: Sesi mempertahankan pesan sebelumnya
* **Dukungan interrupt**: Dapat menghentikan eksekusi di tengah-tengah tugas
* **Siklus hidup eksplisit**: Anda mengontrol kapan sesi dimulai dan berakhir
* **Alur berbasis respons**: Dapat bereaksi terhadap respons dan mengirim tindak lanjut
* **Tools dan hooks kustom**: Mendukung tools kustom (dibuat dengan dekorator `@tool`) dan hooks

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: str) -> None
    async def set_model(self, model: str | None = None) -> None
    async def rewind_files(self, user_message_id: str) -> None
    async def get_mcp_status(self) -> McpStatusResponse
    async def reconnect_mcp_server(self, server_name: str) -> None
    async def toggle_mcp_server(self, server_name: str, enabled: bool) -> None
    async def stop_task(self, task_id: str) -> None
    async def get_server_info(self) -> dict[str, Any] | None
    async def disconnect(self) -> None
```

<h4 id="methods">
  Metode
</h4>

| Metode                                    | Deskripsi                                                                                                                                                                      |
| :---------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Inisialisasi klien dengan konfigurasi opsional                                                                                                                                 |
| `connect(prompt)`                         | Hubungkan ke Claude dengan prompt awal opsional atau aliran pesan                                                                                                              |
| `query(prompt, session_id)`               | Kirim permintaan baru dalam mode streaming                                                                                                                                     |
| `receive_messages()`                      | Terima semua pesan dari Claude sebagai async iterator                                                                                                                          |
| `receive_response()`                      | Terima pesan hingga dan termasuk ResultMessage                                                                                                                                 |
| `interrupt()`                             | Kirim sinyal interrupt (hanya bekerja dalam mode streaming)                                                                                                                    |
| `set_permission_mode(mode)`               | Ubah mode izin untuk sesi saat ini                                                                                                                                             |
| `set_model(model)`                        | Ubah model untuk sesi saat ini. Teruskan `None` untuk reset ke default                                                                                                         |
| `rewind_files(user_message_id)`           | Pulihkan file ke keadaan mereka pada pesan pengguna yang ditentukan. Memerlukan `enable_file_checkpointing=True`. Lihat [File checkpointing](/docs/id/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Dapatkan status semua server MCP yang dikonfigurasi. Mengembalikan [`McpStatusResponse`](#mcpstatusresponse)                                                                   |
| `reconnect_mcp_server(server_name)`       | Coba lagi menghubungkan ke server MCP yang gagal atau terputus                                                                                                                 |
| `toggle_mcp_server(server_name, enabled)` | Aktifkan atau nonaktifkan server MCP di tengah sesi. Menonaktifkan menghapus toolnya                                                                                           |
| `stop_task(task_id)`                      | Hentikan tugas latar belakang yang sedang berjalan. [`TaskNotificationMessage`](#tasknotificationmessage) dengan status `"stopped"` mengikuti dalam aliran pesan               |
| `get_server_info()`                       | Dapatkan informasi server termasuk ID sesi dan kemampuan                                                                                                                       |
| `disconnect()`                            | Putuskan sambungan dari Claude                                                                                                                                                 |

<h4 id="context-manager-support">
  Dukungan Context Manager
</h4>

Klien dapat digunakan sebagai async context manager untuk manajemen koneksi otomatis:

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **Penting:** Saat mengulangi pesan, hindari menggunakan `break` untuk keluar lebih awal karena ini dapat menyebabkan masalah pembersihan asyncio. Sebaliknya, biarkan iterasi selesai secara alami atau gunakan flag untuk melacak kapan Anda menemukan apa yang Anda butuhkan.

<h4 id="example-continuing-a-conversation">
  Contoh - Melanjutkan percakapan
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage


async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - the session retains the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")


asyncio.run(main())
```

<h4 id="example-streaming-input-with-claudesdkclient">
  Contoh - Streaming input dengan ClaudeSDKClient
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def message_stream():
    """Generate messages dynamically."""
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Analyze the following data:"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Temperature: 25°C, Humidity: 60%"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "What patterns do you see?"},
    }


async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

<h4 id="example-using-interrupts">
  Contoh - Menggunakan interrupts
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, ResultMessage


async def interruptible_task():
    options = ClaudeAgentOptions(allowed_tools=["Bash"], permission_mode="acceptEdits")

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly, using the bash sleep command")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Drain the interrupted task's messages (including its ResultMessage)
        async for message in client.receive_response():
            if isinstance(message, ResultMessage):
                print(f"Interrupted task finished with subtype={message.subtype!r}")
                # subtype is "error_during_execution" for interrupted tasks

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **Perilaku buffer setelah interrupt:** `interrupt()` mengirim sinyal berhenti tetapi tidak menghapus buffer pesan. Pesan yang sudah diproduksi oleh tugas yang terputus, termasuk `ResultMessage`-nya (dengan `subtype="error_during_execution"`), tetap berada dalam aliran. Anda harus menguras mereka dengan `receive_response()` sebelum membaca respons ke query baru. Jika Anda mengirim query baru segera setelah `interrupt()` dan memanggil `receive_response()` hanya sekali, Anda akan menerima pesan tugas yang terputus, bukan respons query baru.
</Note>

<h4 id="example-advanced-permission-control">
  Contoh - Kontrol izin lanjutan
</h4>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
from claude_agent_sdk.types import (
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def custom_permission_handler(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return PermissionResultDeny(
            message="System directory write not allowed", interrupt=True
        )

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return PermissionResultAllow(
            updated_input={**input_data, "file_path": safe_path}
        )

    # Allow everything else
    return PermissionResultAllow(updated_input=input_data)


async def main():
    # Jangan juga daftarkan tools yang dibatasi dalam allowed_tools: aturan izin menyetujui panggilan sebelum can_use_tool berjalan
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

<h2 id="types">
  Tipe
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** SDK ini menggunakan dua jenis tipe. Kelas yang didekorasi dengan `@dataclass` (seperti `ResultMessage`, `AgentDefinition`, `TextBlock`) adalah instance objek saat runtime dan mendukung akses atribut: `msg.result`. Kelas yang didefinisikan dengan `TypedDict` (seperti `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) adalah **dict biasa saat runtime** dan memerlukan akses kunci: `config["budget_tokens"]`, bukan `config.budget_tokens`. Sintaks panggilan `ClassName(field=value)` bekerja untuk keduanya, tetapi hanya dataclass yang menghasilkan objek dengan atribut.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Definisi untuk tool SDK MCP yang dibuat dengan dekorator `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Properti       | Tipe                                       | Deskripsi                                                                                                  |
| :------------- | :----------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Pengenal unik untuk tool                                                                                   |
| `description`  | `str`                                      | Deskripsi yang dapat dibaca manusia                                                                        |
| `input_schema` | `type[T] \| dict[str, Any]`                | Skema untuk validasi input                                                                                 |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Fungsi async yang menangani eksekusi tool                                                                  |
| `annotations`  | `ToolAnnotations \| None`                  | Anotasi tool MCP opsional (misalnya, `readOnlyHint`, `destructiveHint`, `openWorldHint`). Dari `mcp.types` |

<h3 id="transport">
  `Transport`
</h3>

Kelas dasar abstrak untuk implementasi transport kustom. Gunakan ini untuk berkomunikasi dengan proses Claude melalui saluran kustom (misalnya, koneksi jarak jauh alih-alih subprocess lokal).

<Warning>
  Ini adalah API internal tingkat rendah. Antarmuka dapat berubah di rilis mendatang. Implementasi kustom harus diperbarui agar sesuai dengan perubahan antarmuka apa pun.
</Warning>

```python theme={null}
from abc import ABC, abstractmethod
from collections.abc import AsyncIterator
from typing import Any


class Transport(ABC):
    @abstractmethod
    async def connect(self) -> None: ...

    @abstractmethod
    async def write(self, data: str) -> None: ...

    @abstractmethod
    def read_messages(self) -> AsyncIterator[dict[str, Any]]: ...

    @abstractmethod
    async def close(self) -> None: ...

    @abstractmethod
    def is_ready(self) -> bool: ...

    @abstractmethod
    async def end_input(self) -> None: ...
```

| Metode            | Deskripsi                                                             |
| :---------------- | :-------------------------------------------------------------------- |
| `connect()`       | Hubungkan transport dan siapkan untuk komunikasi                      |
| `write(data)`     | Tulis data mentah (JSON + newline) ke transport                       |
| `read_messages()` | Async iterator yang menghasilkan pesan JSON yang diuraikan            |
| `close()`         | Tutup koneksi dan bersihkan sumber daya                               |
| `is_ready()`      | Mengembalikan `True` jika transport dapat mengirim dan menerima       |
| `end_input()`     | Tutup aliran input (misalnya, tutup stdin untuk transport subprocess) |

Impor: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass konfigurasi untuk query Claude Code.

```python theme={null}
@dataclass
class ClaudeAgentOptions:
    tools: list[str] | ToolsPreset | None = None
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    max_turns: int | None = None
    max_budget_usd: float | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    fallback_model: str | None = None
    betas: list[SdkBeta] = field(default_factory=list)
    output_format: dict[str, Any] | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    cli_path: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    include_hook_events: bool = False
    fork_session: bool = False
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
    skills: list[str] | Literal["all"] | None = None
    sandbox: SandboxSettings | None = None
    plugins: list[SdkPluginConfig] = field(default_factory=list)
    max_thinking_tokens: int | None = None  # Deprecated: use thinking instead
    thinking: ThinkingConfig | None = None
    effort: EffortLevel | None = None
    enable_file_checkpointing: bool = False
    session_store: SessionStore | None = None
    session_store_flush: SessionStoreFlushMode = "batched"
```

| Properti                      | Tipe                                                                                  | Default                            | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                             | Konfigurasi tools. Gunakan `{"type": "preset", "preset": "claude_code"}` untuk tools default Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                               | Tools untuk auto-approve tanpa prompt. Ini tidak membatasi Claude hanya pada tools ini; tools yang tidak terdaftar jatuh ke `permission_mode` dan `can_use_tool`. Gunakan `disallowed_tools` untuk memblokir tools. Lihat [Permissions](/docs/id/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                      |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                             | Konfigurasi system prompt. Teruskan string untuk prompt kustom, `{"type": "preset", "preset": "claude_code"}` untuk system prompt Claude Code dengan `"append"` opsional, atau `{"type": "file", "path": "..."}` untuk memuat prompt besar dari disk. Lihat [`SystemPromptPreset`](#systempromptpreset) dan [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                          |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                               | Konfigurasi server MCP atau jalur ke file konfigurasi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `strict_mcp_config`           | `bool`                                                                                | `False`                            | Ketika `True`, gunakan hanya server yang diteruskan dalam `mcp_servers` dan abaikan `.mcp.json` proyek, pengaturan pengguna, server MCP yang disediakan plugin, dan [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai). Memetakan ke flag CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                             | Mode izin untuk penggunaan tool                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `continue_conversation`       | `bool`                                                                                | `False`                            | Lanjutkan percakapan terbaru                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `resume`                      | `str \| None`                                                                         | `None`                             | ID sesi untuk dilanjutkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `max_turns`                   | `int \| None`                                                                         | `None`                             | Jumlah maksimal putaran agentic (round trip penggunaan tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                             | Hentikan query ketika estimasi biaya sisi klien mencapai nilai USD ini. Dibandingkan dengan estimasi yang sama seperti `total_cost_usd`; lihat [Track cost and usage](/docs/id/agent-sdk/cost-tracking) untuk peringatan akurasi                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                               | Tools untuk ditolak. Nama bare seperti `"Bash"` menghapus tool dari konteks Claude. Aturan scoped seperti `"Bash(rm *)"` membiarkan tool tersedia dan menolak panggilan yang cocok di setiap mode izin, termasuk `bypassPermissions`. Lihat [Permissions](/docs/id/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                    |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                            | Aktifkan pelacakan perubahan file untuk rewinding. Lihat [File checkpointing](/docs/id/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `model`                       | `str \| None`                                                                         | `None`                             | Alias model Claude atau nama model lengkap. Lihat [nilai yang diterima dan ID khusus penyedia](/docs/id/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `fallback_model`              | `str \| None`                                                                         | `None`                             | Model fallback yang akan digunakan jika model utama gagal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                               | Fitur beta untuk diaktifkan. Lihat [`SdkBeta`](#sdkbeta) untuk opsi yang tersedia                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                             | Format output untuk respons terstruktur (misalnya, `{"type": "json_schema", "schema": {...}}`). Lihat [Structured outputs](/docs/id/agent-sdk/structured-outputs) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                             | Nama tool MCP untuk prompt izin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                             | Direktori kerja saat ini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                             | Jalur kustom ke executable CLI Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `settings`                    | `str \| None`                                                                         | `None`                             | Jalur ke file pengaturan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                               | Direktori tambahan yang dapat diakses Claude                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `env`                         | `dict[str, str]`                                                                      | `{}`                               | Variabel lingkungan yang digabungkan di atas lingkungan proses yang diwarisi. Lihat [Environment variables](/docs/id/env-vars) untuk variabel yang dibaca CLI yang mendasar, dan [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) untuk variabel terkait timeout                                                                                                                                                                                                                                                                                                                                                     |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                               | Argumen CLI tambahan untuk diteruskan langsung ke CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                             | Byte maksimal saat membuffer stdout CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                       | *Deprecated* - Objek seperti file untuk output debug. Gunakan callback `stderr` sebagai gantinya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                             | Fungsi callback untuk output stderr dari CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                             | Fungsi callback izin tool, dipanggil hanya ketika [alur izin](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) jatuh ke prompt. Tidak dipanggil untuk panggilan yang auto-approved oleh entri `allowed_tools`, aturan allow, atau `permission_mode`. `AskUserQuestion`, connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan tool MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) mencapainya bahkan jika Anda telah mengizinkannya; dalam mode `dontAsk` ini ditolak sebagai gantinya. Lihat [`CanUseTool`](#canusetool) untuk detail |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                             | Konfigurasi hook untuk mengintersepsi event                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `user`                        | `str \| None`                                                                         | `None`                             | Pengenal pengguna                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `include_partial_messages`    | `bool`                                                                                | `False`                            | Sertakan event streaming pesan parsial. Ketika diaktifkan, pesan [`StreamEvent`](#streamevent) dihasilkan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `include_hook_events`         | `bool`                                                                                | `False`                            | Sertakan event lifecycle hook dalam aliran pesan sebagai objek `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `fork_session`                | `bool`                                                                                | `False`                            | Ketika melanjutkan dengan `resume`, fork ke ID sesi baru alih-alih melanjutkan sesi asli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                             | Subagent yang didefinisikan secara programatis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                               | Muat plugin kustom dari jalur lokal. Lihat [Plugins](/docs/id/agent-sdk/plugins) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                             | Konfigurasi perilaku sandbox secara programatis. Lihat [Sandbox settings](#sandboxsettings) untuk detail                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI defaults: all sources) | Kontrol pengaturan filesystem mana yang akan dimuat. Teruskan `[]` untuk menonaktifkan pengaturan pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola dimuat terlepas dari itu; pengaturan yang dikelola server diambil ketika sesi mengautentikasi dengan kredensial organisasi pada [konfigurasi yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability). Lihat [Use Claude Code features](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk input yang dibaca terlepas dari opsi ini, dan cara menonaktifkannya.                                                                    |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                             | Skills yang tersedia untuk sesi. Teruskan `"all"` untuk mengaktifkan setiap skill yang ditemukan, atau daftar nama skill. Ketika diatur, SDK secara otomatis menambahkan tool Skill ke `allowed_tools`. Jika Anda juga meneruskan `tools`, sertakan `"Skill"` dalam daftar itu. Lihat [Skills](/docs/id/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                         |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                             | *Deprecated* - Token maksimal untuk blok thinking. Gunakan `thinking` sebagai gantinya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                             | Mengontrol perilaku extended thinking. Mengambil prioritas atas `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                             | Tingkat usaha untuk kedalaman thinking. Lihat [adjust the effort level](/docs/id/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `session_store`               | [`SessionStore`](/docs/id/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                             | Cerminkan transkrip sesi ke backend eksternal sehingga host apa pun dapat melanjutkannya. Lihat [Persist sessions to external storage](/docs/id/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                        | Kapan membuang entri transkrip yang dicerminkan ke `session_store`. `"batched"` membuang sekali per putaran atau ketika buffer penuh; `"eager"` memicu pembilasan latar belakang setelah setiap frame. Diabaikan ketika `session_store` adalah `None`                                                                                                                                                                                                                                                                                                                                                                                        |

<h4 id="handle-slow-or-stalled-api-responses">
  Menangani respons API yang lambat atau terhenti
</h4>

Subprocess CLI membaca beberapa variabel lingkungan yang mengontrol timeout API dan deteksi stall. Teruskan melalui `ClaudeAgentOptions.env`:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: timeout per-permintaan pada klien Anthropic, dalam milidetik. Default `600000`. Berlaku untuk loop utama dan semua subagent.
* `CLAUDE_CODE_MAX_RETRIES`: maksimal retry API. Default `10`, dibatasi pada `15`. Setiap retry mendapat jendela `API_TIMEOUT_MS` sendiri, jadi waktu dinding terburuk kira-kira `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` ditambah backoff. Untuk run tanpa pengawasan yang perlu menunggu melalui pemadaman yang lebih lama, atur `CLAUDE_CODE_RETRY_WATCHDOG=1`: itu retry kapasitas error tanpa batas, dan sejak Claude Code v2.1.199 menaikkan default untuk error transien lainnya menjadi `300` dan menghapus batas pada variabel ini.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog stall untuk subagent yang diluncurkan dengan `run_in_background`. Default `600000`. Direset pada setiap event stream; pada stall itu membatalkan subagent, menandai tugas gagal, dan menampilkan error ke parent dengan hasil parsial apa pun. Tidak berlaku untuk subagent sinkron.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` dengan `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: membatalkan permintaan ketika header telah tiba tetapi badan respons berhenti streaming. Watchdog aktif secara default untuk semua penyedia; atur `CLAUDE_ENABLE_STREAM_WATCHDOG=0` untuk menonaktifkannya. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` default ke `300000` dan diklem ke minimum itu. Permintaan yang dibatalkan melalui jalur retry normal.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Konfigurasi untuk validasi output terstruktur. Teruskan ini sebagai `dict` ke field `output_format` pada `ClaudeAgentOptions`:

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Field    | Diperlukan | Deskripsi                                        |
| :------- | :--------- | :----------------------------------------------- |
| `type`   | Ya         | Harus `"json_schema"` untuk validasi JSON Schema |
| `schema` | Ya         | Definisi JSON Schema untuk validasi output       |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Konfigurasi untuk menggunakan preset system prompt Claude Code dengan penambahan opsional.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| Field                      | Diperlukan | Deskripsi                                                                                                                                                                                                                                                                                                                          |
| :------------------------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Ya         | Harus `"preset"` untuk menggunakan preset system prompt                                                                                                                                                                                                                                                                            |
| `preset`                   | Ya         | Harus `"claude_code"` untuk menggunakan system prompt Claude Code                                                                                                                                                                                                                                                                  |
| `append`                   | Tidak      | Instruksi tambahan untuk ditambahkan ke preset system prompt                                                                                                                                                                                                                                                                       |
| `exclude_dynamic_sections` | Tidak      | Pindahkan konteks per-sesi seperti direktori kerja, status git-repo, dan jalur memori otomatis dari system prompt ke pesan pengguna pertama. Meningkatkan reuse prompt-cache di seluruh pengguna dan mesin. Lihat [Modify system prompts](/docs/id/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Konfigurasi untuk memuat system prompt kustom dari file alih-alih meneruskannya sebagai string. SDK memetakan ini ke flag CLI [`--system-prompt-file`](/docs/id/cli-reference#system-prompt-flags). Gunakan bentuk file ketika prompt besar: SDK meneruskan string `system_prompt` pada argv subprocess CLI, yang tunduk pada batas panjang command-line OS sebelum SDK mengirim permintaan API apa pun. Di Linux satu argumen lebih panjang dari kira-kira 128 KB gagal pada spawn proses dengan `Argument list too long`. Di Windows seluruh command line dibatasi pada kira-kira 32 KB, jadi bentuk string gagal pada ambang yang lebih rendah.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Field  | Diperlukan | Deskripsi                                    |
| :----- | :--------- | :------------------------------------------- |
| `type` | Ya         | Harus `"file"` untuk memuat prompt dari disk |
| `path` | Ya         | Jalur ke file yang berisi system prompt      |

<h3 id="settingsource">
  `SettingSource`
</h3>

Mengontrol sumber konfigurasi berbasis filesystem mana yang dimuat pengaturan SDK.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Nilai       | Deskripsi                                          | Lokasi                        |
| :---------- | :------------------------------------------------- | :---------------------------- |
| `"user"`    | Pengaturan pengguna global                         | `~/.claude/settings.json`     |
| `"project"` | Pengaturan proyek bersama (version controlled)     | `.claude/settings.json`       |
| `"local"`   | Pengaturan proyek lokal (tidak version controlled) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Perilaku default
</h4>

Ketika `setting_sources` dihilangkan atau `None`, `query()` memuat pengaturan filesystem yang sama seperti CLI Claude Code: pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola dimuat dalam semua kasus; pengaturan yang dikelola server diambil ketika sesi mengautentikasi dengan kredensial organisasi pada [konfigurasi yang memenuhi syarat](/docs/id/server-managed-settings#platform-availability). Lihat [What settingSources does not control](/docs/id/agent-sdk/claude-code-features#what-settingsources-does-not-control) untuk input yang dibaca terlepas dari opsi ini, dan cara menonaktifkannya.

<h4 id="why-use-setting_sources">
  Mengapa menggunakan setting\_sources
</h4>

**Nonaktifkan pengaturan filesystem:**

```python theme={null}
# Do not load user, project, or local settings from disk
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=[]
    ),
):
    print(message)
```

<Note>
  Dalam Python SDK 0.1.59 dan lebih awal, daftar kosong diperlakukan sama dengan menghilangkan opsi, jadi `setting_sources=[]` tidak menonaktifkan pengaturan filesystem. Upgrade ke rilis yang lebih baru jika Anda memerlukan daftar kosong untuk berlaku. SDK TypeScript tidak terpengaruh.
</Note>

**Muat semua pengaturan filesystem secara eksplisit:**

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]
    ),
):
    print(message)
```

**Muat hanya sumber pengaturan tertentu:**

```python theme={null}
# Load only project settings, ignore user and local
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    ),
):
    print(message)
```

**Lingkungan testing dan CI:**

```python theme={null}
# Ensure consistent behavior in CI by excluding local settings
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions",
    ),
):
    print(message)
```

**Aplikasi SDK-only:**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
async for message in query(
    prompt="Review this PR",
    options=ClaudeAgentOptions(
        setting_sources=[],
        agents={...},
        mcp_servers={...},
        allowed_tools=["Read", "Grep", "Glob"],
    ),
):
    print(message)
```

**Memuat instruksi proyek CLAUDE.md:**

```python theme={null}
# Load project settings to include CLAUDE.md files
async for message in query(
    prompt="Add a new feature following project conventions",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",  # Use Claude Code's system prompt
        },
        setting_sources=["project"],  # Loads CLAUDE.md from project
        allowed_tools=["Read", "Write", "Edit"],
    ),
):
    print(message)
```

<h4 id="settings-precedence">
  Preseden pengaturan
</h4>

Ketika beberapa sumber dimuat, pengaturan digabungkan dengan preseden ini (tertinggi ke terendah):

1. Pengaturan lokal (`.claude/settings.local.json`)
2. Pengaturan proyek (`.claude/settings.json`)
3. Pengaturan pengguna (`~/.claude/settings.json`)

Opsi programatis seperti `agents` dan `allowed_tools` mengganti pengaturan filesystem pengguna, proyek, dan lokal. Pengaturan kebijakan terkelola mengambil prioritas atas opsi programatis.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Konfigurasi untuk subagent yang didefinisikan secara programatis.

```python theme={null}
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    disallowedTools: list[str] | None = None
    model: str | None = None
    skills: list[str] | None = None
    memory: Literal["user", "project", "local"] | None = None
    mcpServers: list[str | dict[str, Any]] | None = None
    initialPrompt: str | None = None
    maxTurns: int | None = None
    background: bool | None = None
    effort: EffortLevel | int | None = None
    permissionMode: PermissionMode | None = None
```

| Field             | Diperlukan | Deskripsi                                                                                                                                                                                                                       |
| :---------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | Ya         | Deskripsi bahasa alami tentang kapan menggunakan agent ini                                                                                                                                                                      |
| `prompt`          | Ya         | System prompt agent                                                                                                                                                                                                             |
| `tools`           | Tidak      | Array nama tool yang diizinkan. Jika dihilangkan, mewarisi semua tools                                                                                                                                                          |
| `disallowedTools` | Tidak      | Array nama tool untuk dihapus dari set tool agent. Pola tingkat server MCP juga diterima: `mcp__server` atau `mcp__server__*` menghapus setiap tool dari server itu, dan `mcp__*` menghapus setiap tool MCP dari server apa pun |
| `model`           | Tidak      | Penggantian model untuk agent ini. Menerima alias seperti `"sonnet"`, `"opus"`, `"haiku"`, atau `"inherit"`, atau ID model lengkap. Jika dihilangkan, menggunakan model utama                                                   |
| `skills`          | Tidak      | Daftar nama skill untuk preload ke dalam konteks agent saat startup. Skill yang tidak terdaftar tetap dapat dipanggil melalui tool Skill                                                                                        |
| `memory`          | Tidak      | Sumber memori untuk agent ini: `"user"`, `"project"`, atau `"local"`                                                                                                                                                            |
| `mcpServers`      | Tidak      | Server MCP yang tersedia untuk agent ini. Setiap entri adalah nama server atau dict `{name: config}` inline                                                                                                                     |
| `initialPrompt`   | Tidak      | Auto-submitted sebagai putaran pengguna pertama ketika agent ini berjalan sebagai agent thread utama                                                                                                                            |
| `maxTurns`        | Tidak      | Jumlah maksimal putaran agentic sebelum agent berhenti                                                                                                                                                                          |
| `background`      | Tidak      | Jalankan agent ini sebagai tugas latar belakang non-blocking ketika dipanggil                                                                                                                                                   |
| `effort`          | Tidak      | Tingkat usaha reasoning untuk agent ini. Menerima tingkat bernama atau integer. Lihat [`EffortLevel`](#effortlevel)                                                                                                             |
| `permissionMode`  | Tidak      | Mode izin untuk eksekusi tool dalam agent ini. Lihat [`PermissionMode`](#permissionmode)                                                                                                                                        |

<Note>
  Field `AgentDefinition` menggunakan camelCase, seperti `disallowedTools`, `permissionMode`, dan `maxTurns`. Nama-nama ini memetakan langsung ke format wire yang dibagikan dengan SDK TypeScript. Ini berbeda dari `ClaudeAgentOptions`, yang menggunakan Python snake\_case untuk field tingkat atas yang setara seperti `disallowed_tools` dan `permission_mode`. Karena `AgentDefinition` adalah dataclass, melewatkan keyword snake\_case menimbulkan `TypeError` pada waktu konstruksi.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Mode izin untuk mengontrol eksekusi tool.

```python theme={null}
PermissionMode = Literal[
    "default",  # Standard permission behavior
    "acceptEdits",  # Auto-accept file edits
    "plan",  # Planning mode - explore without editing
    "dontAsk",  # Deny anything not pre-approved instead of prompting
    "bypassPermissions",  # Bypass permission checks; explicit ask rules still prompt (use with caution)
    "auto",  # A model classifier approves or denies each tool call
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

Tingkat usaha untuk membimbing kedalaman thinking.

```python theme={null}
EffortLevel = Literal[
    "low",  # Minimal thinking, fastest responses
    "medium",  # Moderate thinking
    "high",  # Deep reasoning
    "xhigh",  # Extended reasoning; falls back to "high" on models that don't support it
    "max",  # Maximum effort
]
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Type alias untuk fungsi callback izin tool.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

Callback menerima:

* `tool_name`: Nama tool yang sedang dipanggil
* `input_data`: Parameter input tool
* `context`: `ToolPermissionContext` dengan informasi tambahan

Mengembalikan `PermissionResult` (baik `PermissionResultAllow` atau `PermissionResultDeny`).

Callback adalah pengganti SDK untuk prompt izin interaktif: dipanggil hanya ketika [alur evaluasi izin](/docs/id/agent-sdk/permissions#how-permissions-are-evaluated) diselesaikan ke prompt. Panggilan tool yang sudah disetujui oleh entri `allowed_tools`, aturan allow pengaturan, atau mode izin, seperti `acceptEdits` atau `bypassPermissions`, tidak pernah memanggilnya. Untuk gating setiap panggilan tool, gunakan hook [`PreToolUse`](/docs/id/agent-sdk/hooks) sebagai gantinya.

`AskUserQuestion`, tool MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan connector tools [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) mencapai callback bahkan ketika aturan allow cocok. Dalam mode `dontAsk` panggilan ini ditolak sebagai gantinya, tanpa memanggil callback.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Informasi konteks yang diteruskan ke callback izin tool.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| Field             | Tipe                     | Deskripsi                                                                                                                                                                                                                      |
| :---------------- | :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Dicadangkan untuk dukungan sinyal abort di masa depan                                                                                                                                                                          |
| `suggestions`     | `list[PermissionUpdate]` | Saran pembaruan izin dari CLI. Prompt Bash menyertakan saran dengan destinasi `localSettings`, jadi mengembalikannya dalam `updated_permissions` menulis aturan ke `.claude/settings.local.json` dan bertahan di seluruh sesi. |
| `blocked_path`    | `str \| None`            | Jalur file yang memicu permintaan izin, jika berlaku. Misalnya, ketika perintah Bash mencoba mengakses jalur di luar direktori yang diizinkan                                                                                  |
| `decision_reason` | `str \| None`            | Alasan permintaan izin ini dipicu. Diteruskan dari `permissionDecisionReason` hook PreToolUse ketika hook mengembalikan `"ask"`                                                                                                |
| `title`           | `str \| None`            | Kalimat prompt izin lengkap, seperti `Claude wants to read foo.txt`. Gunakan sebagai teks prompt utama ketika ada                                                                                                              |
| `display_name`    | `str \| None`            | Frasa kata benda pendek untuk aksi tool, seperti `Read file`, cocok untuk label tombol                                                                                                                                         |
| `description`     | `str \| None`            | Subtitle yang dapat dibaca manusia untuk UI izin                                                                                                                                                                               |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Tipe union untuk hasil callback izin.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Hasil yang menunjukkan panggilan tool harus diizinkan.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Field                 | Tipe                             | Default   | Deskripsi                                              |
| :-------------------- | :------------------------------- | :-------- | :----------------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"` | Harus "allow"                                          |
| `updated_input`       | `dict[str, Any] \| None`         | `None`    | Input yang dimodifikasi untuk digunakan alih-alih asli |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`    | Pembaruan izin untuk diterapkan                        |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Hasil yang menunjukkan panggilan tool harus ditolak.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Field       | Tipe              | Default  | Deskripsi                                   |
| :---------- | :---------------- | :------- | :------------------------------------------ |
| `behavior`  | `Literal["deny"]` | `"deny"` | Harus "deny"                                |
| `message`   | `str`             | `""`     | Pesan yang menjelaskan mengapa tool ditolak |
| `interrupt` | `bool`            | `False`  | Apakah akan mengganggu eksekusi saat ini    |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Konfigurasi untuk memperbarui izin secara programatis.

```python theme={null}
@dataclass
class PermissionUpdate:
    type: Literal[
        "addRules",
        "replaceRules",
        "removeRules",
        "setMode",
        "addDirectories",
        "removeDirectories",
    ]
    rules: list[PermissionRuleValue] | None = None
    behavior: Literal["allow", "deny", "ask"] | None = None
    mode: PermissionMode | None = None
    directories: list[str] | None = None
    destination: (
        Literal["userSettings", "projectSettings", "localSettings", "session"] | None
    ) = None
```

| Field         | Tipe                                      | Deskripsi                                    |
| :------------ | :---------------------------------------- | :------------------------------------------- |
| `type`        | `Literal[...]`                            | Jenis operasi pembaruan izin                 |
| `rules`       | `list[PermissionRuleValue] \| None`       | Aturan untuk operasi add/replace/remove      |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Perilaku untuk operasi berbasis aturan       |
| `mode`        | `PermissionMode \| None`                  | Mode untuk operasi setMode                   |
| `directories` | `list[str] \| None`                       | Direktori untuk operasi add/remove direktori |
| `destination` | `Literal[...] \| None`                    | Di mana menerapkan pembaruan izin            |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Aturan untuk ditambahkan, diganti, atau dihapus dalam pembaruan izin.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Konfigurasi preset tools untuk menggunakan set tool default Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Mengontrol perilaku extended thinking. Union dari tiga konfigurasi:

```python theme={null}
ThinkingDisplay = Literal["summarized", "omitted"]


class ThinkingConfigAdaptive(TypedDict):
    type: Literal["adaptive"]
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigEnabled(TypedDict):
    type: Literal["enabled"]
    budget_tokens: int
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigDisabled(TypedDict):
    type: Literal["disabled"]


ThinkingConfig = ThinkingConfigAdaptive | ThinkingConfigEnabled | ThinkingConfigDisabled
```

| Varian     | Field                              | Deskripsi                                             |
| :--------- | :--------------------------------- | :---------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude secara adaptif memutuskan kapan harus berpikir |
| `enabled`  | `type`, `budget_tokens`, `display` | Aktifkan thinking dengan budget token tertentu        |
| `disabled` | `type`                             | Nonaktifkan thinking                                  |

Field opsional `display` mengontrol apakah teks thinking dikembalikan `"summarized"` atau `"omitted"`. Pada Claude Opus 4.7 dan lebih baru, default API adalah `"omitted"`, jadi atur `"summarized"` untuk menerima konten thinking dalam output [`ThinkingBlock`](#thinkingblock).

Karena ini adalah kelas `TypedDict`, mereka adalah dict biasa saat runtime. Baik buatlah sebagai dict literal atau panggil kelas seperti konstruktor; keduanya menghasilkan `dict`. Akses field dengan `config["budget_tokens"]`, bukan `config.budget_tokens`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Option 1: dict literal (recommended, no import needed)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Option 2: constructor-style (returns a plain dict)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens would raise AttributeError
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Tipe literal untuk fitur beta SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Gunakan dengan field `betas` dalam `ClaudeAgentOptions` untuk mengaktifkan fitur beta.

<Warning>
  Beta `context-1m-2025-08-07` sudah pensiun sejak 30 April 2026. Melewatkan header ini dengan Claude Sonnet 4.5 atau Sonnet 4 tidak berpengaruh, dan permintaan yang melebihi jendela konteks standar 200k-token mengembalikan error. Untuk menggunakan jendela konteks 1M-token, migrasikan ke [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, atau Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), yang mencakup konteks 1M pada harga standar tanpa header beta yang diperlukan.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Konfigurasi untuk server MCP SDK yang dibuat dengan `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Tipe union untuk konfigurasi server MCP.

```python theme={null}
McpServerConfig = (
    McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
)
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```python theme={null}
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```python theme={null}
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```python theme={null}
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

Konfigurasi server MCP seperti yang dilaporkan oleh [`get_mcp_status()`](#methods). Ini adalah union dari semua varian transport [`McpServerConfig`](#mcpserverconfig) ditambah varian output-only `claudeai-proxy` untuk server yang di-proxy melalui claude.ai.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` adalah bentuk yang dapat diserialisasi dari [`McpSdkServerConfig`](#mcpsdkserverconfig) dengan hanya field `type` (`"sdk"`) dan `name` (`str`); `instance` dalam proses dihilangkan. `McpClaudeAIProxyServerConfig` memiliki field `type` (`"claudeai-proxy"`), `url` (`str`), dan `id` (`str`).

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Respons dari [`ClaudeSDKClient.get_mcp_status()`](#methods). Membungkus daftar status server di bawah kunci `mcpServers`.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Status server MCP yang terhubung, terdapat dalam [`McpStatusResponse`](#mcpstatusresponse).

```python theme={null}
class McpServerStatus(TypedDict):
    name: str
    status: McpServerConnectionStatus  # "connected" | "failed" | "needs-auth" | "pending" | "disabled"
    serverInfo: NotRequired[McpServerInfo]
    error: NotRequired[str]
    config: NotRequired[McpServerStatusConfig]
    scope: NotRequired[str]
    tools: NotRequired[list[McpToolInfo]]
```

| Field        | Tipe                                                         | Deskripsi                                                                                                                                                                                       |
| :----------- | :----------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                        | Nama server                                                                                                                                                                                     |
| `status`     | `str`                                                        | Salah satu dari `"connected"`, `"failed"`, `"needs-auth"`, `"pending"`, atau `"disabled"`                                                                                                       |
| `serverInfo` | `dict` (opsional)                                            | Nama dan versi server (`{"name": str, "version": str}`)                                                                                                                                         |
| `error`      | `str` (opsional)                                             | Pesan error jika server gagal terhubung                                                                                                                                                         |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (opsional) | Konfigurasi server. Bentuk yang sama seperti [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP, atau SDK), ditambah varian `claudeai-proxy` untuk server yang terhubung melalui claude.ai |
| `scope`      | `str` (opsional)                                             | Scope konfigurasi                                                                                                                                                                               |
| `tools`      | `list` (opsional)                                            | Tools yang disediakan oleh server ini, masing-masing dengan field `name`, `description`, dan `annotations`                                                                                      |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Konfigurasi untuk memuat plugins dalam SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Field  | Tipe               | Deskripsi                                                    |
| :----- | :----------------- | :----------------------------------------------------------- |
| `type` | `Literal["local"]` | Harus `"local"` (hanya plugins lokal yang saat ini didukung) |
| `path` | `str`              | Jalur absolut atau relatif ke direktori plugin               |

**Contoh:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Untuk informasi lengkap tentang membuat dan menggunakan plugins, lihat [Plugins](/docs/id/agent-sdk/plugins).

<h2 id="message-types">
  Tipe Pesan
</h2>

<h3 id="message">
  `Message`
</h3>

Tipe union dari semua pesan yang mungkin.

```python theme={null}
Message = (
    UserMessage
    | AssistantMessage
    | SystemMessage
    | ResultMessage
    | StreamEvent
    | RateLimitEvent
)
```

<h3 id="usermessage">
  `UserMessage`
</h3>

Pesan input pengguna.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| Field                | Tipe                        | Deskripsi                                                   |
| :------------------- | :-------------------------- | :---------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Konten pesan sebagai teks atau blok konten                  |
| `uuid`               | `str \| None`               | Pengenal pesan unik                                         |
| `parent_tool_use_id` | `str \| None`               | ID penggunaan tool jika pesan ini adalah respons hasil tool |
| `tool_use_result`    | `dict[str, Any] \| None`    | Data hasil tool jika berlaku                                |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Pesan respons asisten dengan blok konten.

```python theme={null}
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
    parent_tool_use_id: str | None = None
    error: AssistantMessageError | None = None
    usage: dict[str, Any] | None = None
    message_id: str | None = None
```

| Field                | Tipe                                                         | Deskripsi                                                                                    |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | Daftar blok konten dalam respons                                                             |
| `model`              | `str`                                                        | Model yang menghasilkan respons                                                              |
| `parent_tool_use_id` | `str \| None`                                                | ID penggunaan tool jika ini adalah respons bersarang                                         |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Tipe error jika respons mengalami error                                                      |
| `usage`              | `dict[str, Any] \| None`                                     | Penggunaan token per-pesan (kunci yang sama seperti [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID pesan API. Beberapa pesan dari satu putaran berbagi ID yang sama                          |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Tipe error yang mungkin untuk pesan asisten.

```python theme={null}
AssistantMessageError = Literal[
    "authentication_failed",
    "billing_error",
    "rate_limit",
    "invalid_request",
    "server_error",
    "max_output_tokens",
    "unknown",
]
```

<h3 id="systemmessage">
  `SystemMessage`
</h3>

Pesan sistem dengan metadata.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Pesan hasil akhir dengan informasi biaya dan penggunaan.

```python theme={null}
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    stop_reason: str | None = None
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
    structured_output: Any = None
    model_usage: dict[str, Any] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
```

Field `subtype` menentukan field mana yang lainnya diisi. Ini adalah salah satu dari `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"`, atau `"error_max_structured_output_retries"`. Dataclass Python meratakan semua varian menjadi satu bentuk, jadi field yang tidak berlaku untuk subtype yang dikembalikan adalah `None`.

Beberapa field membawa detail diagnostik ketika percakapan berakhir dengan error:

* `is_error`: `True` ketika percakapan berakhir dalam status error. Selalu `True` pada subtype `error_*`. Pada `subtype="success"` ini adalah `True` ketika permintaan model terakhir gagal, berarti loop agen selesai tetapi panggilan API terakhir mengembalikan error.
* `api_error_status`: kode status HTTP dari error API yang mengakhiri. `None` ketika putaran berakhir tanpa satu. Diisi hanya pada `subtype="success"`.
* `result`: teks pesan asisten terakhir pada `subtype="success"`, atau `None` pada subtype `error_*`. Ketika `subtype="success"` dan `is_error=True`, ini menyimpan string error API jika tersedia tetapi dapat kosong, jadi periksa `api_error_status` dan konten `AssistantMessage` sebelumnya untuk detail.
* `errors`: string error tingkat loop seperti pesan max-turns. Diisi hanya pada subtype `error_*`.

Dict `usage` berisi kunci berikut ketika ada:

| Kunci                         | Tipe  | Deskripsi                                                                                                                                                                                                   |
| ----------------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Token input yang dikonsumsi oleh loop agen tingkat atas. [Token subagent tidak disertakan](/docs/id/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); gunakan `model_usage` untuk akuntansi seluruh pohon. |
| `output_tokens`               | `int` | Token output yang dihasilkan oleh loop agen tingkat atas. Token subagent tidak disertakan.                                                                                                                  |
| `cache_creation_input_tokens` | `int` | Token yang digunakan untuk membuat entri cache baru.                                                                                                                                                        |
| `cache_read_input_tokens`     | `int` | Token yang dibaca dari entri cache yang ada.                                                                                                                                                                |

Dict `model_usage` memetakan nama model ke penggunaan per-model. Kunci dict dalam menggunakan camelCase karena nilai diteruskan tanpa modifikasi dari proses CLI yang mendasar, cocok dengan tipe [`ModelUsage`](/docs/id/agent-sdk/typescript#modelusage) TypeScript:

| Kunci                      | Tipe    | Deskripsi                                                                                                                                                     |
| -------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Token input untuk model ini.                                                                                                                                  |
| `outputTokens`             | `int`   | Token output untuk model ini.                                                                                                                                 |
| `cacheReadInputTokens`     | `int`   | Token baca cache untuk model ini.                                                                                                                             |
| `cacheCreationInputTokens` | `int`   | Token pembuatan cache untuk model ini.                                                                                                                        |
| `webSearchRequests`        | `int`   | Permintaan pencarian web yang dibuat oleh model ini.                                                                                                          |
| `costUSD`                  | `float` | Biaya yang diperkirakan dalam USD untuk model ini, dihitung sisi klien. Lihat [Track cost and usage](/docs/id/agent-sdk/cost-tracking) untuk peringatan penagihan. |
| `contextWindow`            | `int`   | Ukuran jendela konteks untuk model ini.                                                                                                                       |
| `maxOutputTokens`          | `int`   | Batas token output maksimal untuk model ini.                                                                                                                  |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Event stream untuk pembaruan pesan parsial selama streaming. Hanya diterima ketika `include_partial_messages=True` dalam `ClaudeAgentOptions`. Impor via `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Field                | Tipe             | Deskripsi                                                                                                                                                     |
| :------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `uuid`               | `str`            | Pengenal unik untuk event ini                                                                                                                                 |
| `session_id`         | `str`            | Pengenal sesi                                                                                                                                                 |
| `event`              | `dict[str, Any]` | Data event stream Claude API mentah                                                                                                                           |
| `parent_tool_use_id` | `str \| None`    | Selalu `None`. Event stream dipancarkan untuk sesi utama saja. Untuk atribusi subagent, gunakan pesan lengkap seperti [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Dipancarkan ketika status rate limit berubah (misalnya, dari `"allowed"` ke `"allowed_warning"`). Gunakan ini untuk memperingatkan pengguna sebelum mereka mencapai batas keras, atau untuk mundur ketika status adalah `"rejected"`.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Field             | Tipe                              | Deskripsi                  |
| :---------------- | :-------------------------------- | :------------------------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | Status rate limit saat ini |
| `uuid`            | `str`                             | Pengenal event unik        |
| `session_id`      | `str`                             | Pengenal sesi              |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

Status rate limit yang dibawa oleh [`RateLimitEvent`](#ratelimitevent).

```python theme={null}
RateLimitStatus = Literal["allowed", "allowed_warning", "rejected"]
RateLimitType = Literal[
    "five_hour", "seven_day", "seven_day_opus", "seven_day_sonnet", "overage"
]


@dataclass
class RateLimitInfo:
    status: RateLimitStatus
    resets_at: int | None = None
    rate_limit_type: RateLimitType | None = None
    utilization: float | None = None
    overage_status: RateLimitStatus | None = None
    overage_resets_at: int | None = None
    overage_disabled_reason: str | None = None
    raw: dict[str, Any] = field(default_factory=dict)
```

| Field                     | Tipe                      | Deskripsi                                                                                         |
| :------------------------ | :------------------------ | :------------------------------------------------------------------------------------------------ |
| `status`                  | `RateLimitStatus`         | Status saat ini. `"allowed_warning"` berarti mendekati batas; `"rejected"` berarti batas tercapai |
| `resets_at`               | `int \| None`             | Timestamp Unix ketika jendela rate limit direset                                                  |
| `rate_limit_type`         | `RateLimitType \| None`   | Jendela rate limit mana yang berlaku                                                              |
| `utilization`             | `float \| None`           | Fraksi rate limit yang dikonsumsi (0.0 hingga 1.0)                                                |
| `overage_status`          | `RateLimitStatus \| None` | Status penggunaan overage pay-as-you-go, jika berlaku                                             |
| `overage_resets_at`       | `int \| None`             | Timestamp Unix ketika jendela overage direset                                                     |
| `overage_disabled_reason` | `str \| None`             | Mengapa overage tidak tersedia, jika status adalah `"rejected"`                                   |
| `raw`                     | `dict[str, Any]`          | Dict mentah lengkap dari CLI, termasuk field yang tidak dimodelkan di atas                        |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Dipancarkan ketika tugas latar belakang dimulai. Tugas latar belakang adalah apa pun yang dilacak di luar putaran utama: perintah Bash yang di-background, Monitor watch, subagent yang dihasilkan melalui tool Agent, atau agent jarak jauh. Field `task_type` memberi tahu Anda yang mana. Penamaan ini tidak terkait dengan penggantian nama tool `Task`-ke-`Agent`.

```python theme={null}
@dataclass
class TaskStartedMessage(SystemMessage):
    task_id: str
    description: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    task_type: str | None = None
```

| Field         | Tipe          | Deskripsi                                                                                                                       |
| :------------ | :------------ | :------------------------------------------------------------------------------------------------------------------------------ |
| `task_id`     | `str`         | Pengenal unik untuk tugas                                                                                                       |
| `description` | `str`         | Deskripsi tugas                                                                                                                 |
| `uuid`        | `str`         | Pengenal pesan unik                                                                                                             |
| `session_id`  | `str`         | Pengenal sesi                                                                                                                   |
| `tool_use_id` | `str \| None` | ID penggunaan tool yang terkait                                                                                                 |
| `task_type`   | `str \| None` | Jenis tugas latar belakang: `"local_bash"` untuk Bash dan Monitor watches di background, `"local_agent"`, atau `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Data token dan timing untuk tugas latar belakang.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Dipancarkan secara berkala dengan pembaruan kemajuan untuk tugas latar belakang yang sedang berjalan.

```python theme={null}
@dataclass
class TaskProgressMessage(SystemMessage):
    task_id: str
    description: str
    usage: TaskUsage
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    last_tool_name: str | None = None
```

| Field            | Tipe          | Deskripsi                                   |
| :--------------- | :------------ | :------------------------------------------ |
| `task_id`        | `str`         | Pengenal unik untuk tugas                   |
| `description`    | `str`         | Deskripsi status saat ini                   |
| `usage`          | `TaskUsage`   | Penggunaan token untuk tugas ini sejauh ini |
| `uuid`           | `str`         | Pengenal pesan unik                         |
| `session_id`     | `str`         | Pengenal sesi                               |
| `tool_use_id`    | `str \| None` | ID penggunaan tool yang terkait             |
| `last_tool_name` | `str \| None` | Nama tool terakhir yang digunakan tugas     |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Dipancarkan ketika tugas latar belakang selesai, gagal, atau dihentikan. Tugas latar belakang termasuk perintah Bash `run_in_background`, Monitor watches, dan subagent latar belakang.

```python theme={null}
@dataclass
class TaskNotificationMessage(SystemMessage):
    task_id: str
    status: TaskNotificationStatus  # "completed" | "failed" | "stopped"
    output_file: str
    summary: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    usage: TaskUsage | None = None
```

| Field         | Tipe                     | Deskripsi                                                   |
| :------------ | :----------------------- | :---------------------------------------------------------- |
| `task_id`     | `str`                    | Pengenal unik untuk tugas                                   |
| `status`      | `TaskNotificationStatus` | Salah satu dari `"completed"`, `"failed"`, atau `"stopped"` |
| `output_file` | `str`                    | Jalur ke file output tugas                                  |
| `summary`     | `str`                    | Ringkasan hasil tugas                                       |
| `uuid`        | `str`                    | Pengenal pesan unik                                         |
| `session_id`  | `str`                    | Pengenal sesi                                               |
| `tool_use_id` | `str \| None`            | ID penggunaan tool yang terkait                             |
| `usage`       | `TaskUsage \| None`      | Penggunaan token akhir untuk tugas                          |

<h2 id="content-block-types">
  Tipe Blok Konten
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Tipe union dari semua blok konten.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

Blok konten teks.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Blok konten thinking (untuk model dengan kemampuan thinking).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Blok permintaan penggunaan tool.

```python theme={null}
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

<h3 id="toolresultblock">
  `ToolResultBlock`
</h3>

Blok hasil eksekusi tool.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Tipe Error
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Kelas exception dasar untuk semua error SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Diangkat ketika Claude Code CLI tidak diinstal atau tidak ditemukan.

```python theme={null}
class CLINotFoundError(CLIConnectionError):
    def __init__(
        self, message: str = "Claude Code not found", cli_path: str | None = None
    ):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

<h3 id="cliconnectionerror">
  `CLIConnectionError`
</h3>

Diangkat ketika koneksi ke Claude Code gagal.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Diangkat ketika proses Claude Code gagal.

```python theme={null}
class ProcessError(ClaudeSDKError):
    def __init__(
        self, message: str, exit_code: int | None = None, stderr: str | None = None
    ):
        self.exit_code = exit_code
        self.stderr = stderr
```

<h3 id="clijsondecodeerror">
  `CLIJSONDecodeError`
</h3>

Diangkat ketika parsing JSON gagal.

```python theme={null}
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

<h2 id="hook-types">
  Tipe Hook
</h2>

Untuk panduan komprehensif tentang menggunakan hooks dengan contoh dan pola umum, lihat [Hooks guide](/docs/id/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Tipe event hook yang didukung.

```python theme={null}
HookEvent = Literal[
    "PreToolUse",  # Called before tool execution
    "PostToolUse",  # Called after tool execution
    "PostToolUseFailure",  # Called when a tool execution fails
    "UserPromptSubmit",  # Called when user submits a prompt
    "Stop",  # Called when stopping execution
    "SubagentStop",  # Called when a subagent stops
    "PreCompact",  # Called before message compaction
    "Notification",  # Called for notification events
    "SubagentStart",  # Called when a subagent starts
    "PermissionRequest",  # Called when a permission decision is needed
]
```

<Note>
  SDK TypeScript mendukung event hook tambahan yang belum tersedia di Python: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch`, dan `MessageDisplay`.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Definisi tipe untuk fungsi callback hook.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Parameter:

* `input`: Input hook yang kuat dengan union yang dibedakan berdasarkan `hook_event_name` (lihat [`HookInput`](#hookinput))
* `tool_use_id`: Pengenal penggunaan tool opsional (untuk hook terkait tool)
* `context`: Konteks hook dengan informasi tambahan

Mengembalikan [`HookJSONOutput`](#hookjsonoutput) yang mungkin berisi:

* `decision`: `"block"` untuk memblokir tindakan
* `systemMessage`: Pesan peringatan yang ditampilkan kepada pengguna
* `hookSpecificOutput`: Data output spesifik hook

<h3 id="hookcontext">
  `HookContext`
</h3>

Informasi konteks yang diteruskan ke callback hook.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Konfigurasi untuk mencocokkan hooks ke event atau tools tertentu.

```python theme={null}
@dataclass
class HookMatcher:
    matcher: str | None = (
        None  # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    )
    hooks: list[HookCallback] = field(
        default_factory=list
    )  # List of callbacks to execute
    timeout: float | None = (
        None  # Timeout in seconds for all hooks in this matcher (default: 60)
    )
```

<h3 id="hookinput">
  `HookInput`
</h3>

Tipe union dari semua tipe input hook. Tipe aktual bergantung pada field `hook_event_name`.

```python theme={null}
HookInput = (
    PreToolUseHookInput
    | PostToolUseHookInput
    | PostToolUseFailureHookInput
    | UserPromptSubmitHookInput
    | StopHookInput
    | SubagentStopHookInput
    | PreCompactHookInput
    | NotificationHookInput
    | SubagentStartHookInput
    | PermissionRequestHookInput
)
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Field dasar yang ada di semua tipe input hook.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Field             | Tipe             | Deskripsi                    |
| :---------------- | :--------------- | :--------------------------- |
| `session_id`      | `str`            | Pengenal sesi saat ini       |
| `transcript_path` | `str`            | Jalur ke file transkrip sesi |
| `cwd`             | `str`            | Direktori kerja saat ini     |
| `permission_mode` | `str` (opsional) | Mode izin saat ini           |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Data input untuk event hook `PreToolUse`.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Field             | Tipe                    | Deskripsi                                                    |
| :---------------- | :---------------------- | :----------------------------------------------------------- |
| `hook_event_name` | `Literal["PreToolUse"]` | Selalu "PreToolUse"                                          |
| `tool_name`       | `str`                   | Nama tool yang akan dieksekusi                               |
| `tool_input`      | `dict[str, Any]`        | Parameter input untuk tool                                   |
| `tool_use_id`     | `str`                   | Pengenal unik untuk penggunaan tool ini                      |
| `agent_id`        | `str` (opsional)        | Pengenal subagent, ada ketika hook menyala di dalam subagent |
| `agent_type`      | `str` (opsional)        | Tipe subagent, ada ketika hook menyala di dalam subagent     |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Data input untuk event hook `PostToolUse`.

```python theme={null}
class PostToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_response: Any
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Field             | Tipe                     | Deskripsi                                                    |
| :---------------- | :----------------------- | :----------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUse"]` | Selalu "PostToolUse"                                         |
| `tool_name`       | `str`                    | Nama tool yang dieksekusi                                    |
| `tool_input`      | `dict[str, Any]`         | Parameter input yang digunakan                               |
| `tool_response`   | `Any`                    | Respons dari eksekusi tool                                   |
| `tool_use_id`     | `str`                    | Pengenal unik untuk penggunaan tool ini                      |
| `agent_id`        | `str` (opsional)         | Pengenal subagent, ada ketika hook menyala di dalam subagent |
| `agent_type`      | `str` (opsional)         | Tipe subagent, ada ketika hook menyala di dalam subagent     |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Data input untuk event hook `PostToolUseFailure`. Dipanggil ketika eksekusi tool gagal.

```python theme={null}
class PostToolUseFailureHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUseFailure"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    error: str
    is_interrupt: NotRequired[bool]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Field             | Tipe                            | Deskripsi                                                    |
| :---------------- | :------------------------------ | :----------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Selalu "PostToolUseFailure"                                  |
| `tool_name`       | `str`                           | Nama tool yang gagal                                         |
| `tool_input`      | `dict[str, Any]`                | Parameter input yang digunakan                               |
| `tool_use_id`     | `str`                           | Pengenal unik untuk penggunaan tool ini                      |
| `error`           | `str`                           | Pesan error dari eksekusi yang gagal                         |
| `is_interrupt`    | `bool` (opsional)               | Apakah kegagalan disebabkan oleh interrupt                   |
| `agent_id`        | `str` (opsional)                | Pengenal subagent, ada ketika hook menyala di dalam subagent |
| `agent_type`      | `str` (opsional)                | Tipe subagent, ada ketika hook menyala di dalam subagent     |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Data input untuk event hook `UserPromptSubmit`.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Field             | Tipe                          | Deskripsi                       |
| :---------------- | :---------------------------- | :------------------------------ |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Selalu "UserPromptSubmit"       |
| `prompt`          | `str`                         | Prompt yang dikirimkan pengguna |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Data input untuk event hook `Stop`.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Field              | Tipe              | Deskripsi              |
| :----------------- | :---------------- | :--------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Selalu "Stop"          |
| `stop_hook_active` | `bool`            | Apakah stop hook aktif |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Data input untuk event hook `SubagentStop`.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Field                   | Tipe                      | Deskripsi                        |
| :---------------------- | :------------------------ | :------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Selalu "SubagentStop"            |
| `stop_hook_active`      | `bool`                    | Apakah stop hook aktif           |
| `agent_id`              | `str`                     | Pengenal unik untuk subagent     |
| `agent_transcript_path` | `str`                     | Jalur ke file transkrip subagent |
| `agent_type`            | `str`                     | Tipe subagent                    |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Data input untuk event hook `PreCompact`.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Field                 | Tipe                        | Deskripsi                        |
| :-------------------- | :-------------------------- | :------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Selalu "PreCompact"              |
| `trigger`             | `Literal["manual", "auto"]` | Apa yang memicu pemadatan        |
| `custom_instructions` | `str \| None`               | Instruksi kustom untuk pemadatan |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Data input untuk event hook `Notification`.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Field               | Tipe                      | Deskripsi               |
| :------------------ | :------------------------ | :---------------------- |
| `hook_event_name`   | `Literal["Notification"]` | Selalu "Notification"   |
| `message`           | `str`                     | Konten pesan notifikasi |
| `title`             | `str` (opsional)          | Judul notifikasi        |
| `notification_type` | `str`                     | Tipe notifikasi         |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Data input untuk event hook `SubagentStart`.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Field             | Tipe                       | Deskripsi                    |
| :---------------- | :------------------------- | :--------------------------- |
| `hook_event_name` | `Literal["SubagentStart"]` | Selalu "SubagentStart"       |
| `agent_id`        | `str`                      | Pengenal unik untuk subagent |
| `agent_type`      | `str`                      | Tipe subagent                |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Data input untuk event hook `PermissionRequest`. Memungkinkan hooks untuk menangani keputusan izin secara programatis.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Field                    | Tipe                           | Deskripsi                                                    |
| :----------------------- | :----------------------------- | :----------------------------------------------------------- |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Selalu "PermissionRequest"                                   |
| `tool_name`              | `str`                          | Nama tool yang meminta izin                                  |
| `tool_input`             | `dict[str, Any]`               | Parameter input untuk tool                                   |
| `permission_suggestions` | `list[Any]` (opsional)         | Saran pembaruan izin dari CLI                                |
| `agent_id`               | `str` (opsional)               | Pengenal subagent, ada ketika hook menyala di dalam subagent |
| `agent_type`             | `str` (opsional)               | Tipe subagent, ada ketika hook menyala di dalam subagent     |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Tipe union untuk nilai pengembalian callback hook.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Output hook sinkron dengan field kontrol dan keputusan.

```python theme={null}
class SyncHookJSONOutput(TypedDict):
    # Control fields
    continue_: NotRequired[bool]  # Whether to proceed (default: True)
    suppressOutput: NotRequired[bool]  # Hide stdout from transcript
    stopReason: NotRequired[str]  # Message when continue is False

    # Decision fields
    decision: NotRequired[Literal["block"]]
    systemMessage: NotRequired[str]  # Warning message for user
    reason: NotRequired[str]  # Feedback for Claude

    # Hook-specific output
    hookSpecificOutput: NotRequired[HookSpecificOutput]
```

<Note>
  Gunakan `continue_` (dengan underscore) dalam kode Python. Ini secara otomatis dikonversi ke `continue` ketika dikirim ke CLI.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

`TypedDict` yang berisi nama event hook dan field spesifik event. Bentuknya bergantung pada nilai `hookEventName`. Untuk detail lengkap tentang field yang tersedia per event hook, lihat [Control execution with hooks](/docs/id/agent-sdk/hooks#outputs).

Union yang dibedakan dari tipe output spesifik event. Field `hookEventName` menentukan field mana yang valid.

```python theme={null}
class PreToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PreToolUse"]
    permissionDecision: NotRequired[Literal["allow", "deny", "ask", "defer"]]
    permissionDecisionReason: NotRequired[str]
    updatedInput: NotRequired[dict[str, Any]]
    additionalContext: NotRequired[str]


class PostToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUse"]
    additionalContext: NotRequired[str]
    updatedToolOutput: NotRequired[Any]
    updatedMCPToolOutput: NotRequired[Any]  # Deprecated: use updatedToolOutput, which works for all tools


class PostToolUseFailureHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUseFailure"]
    additionalContext: NotRequired[str]


class UserPromptSubmitHookSpecificOutput(TypedDict):
    hookEventName: Literal["UserPromptSubmit"]
    additionalContext: NotRequired[str]


class NotificationHookSpecificOutput(TypedDict):
    hookEventName: Literal["Notification"]
    additionalContext: NotRequired[str]


class SubagentStartHookSpecificOutput(TypedDict):
    hookEventName: Literal["SubagentStart"]
    additionalContext: NotRequired[str]


class PermissionRequestHookSpecificOutput(TypedDict):
    hookEventName: Literal["PermissionRequest"]
    decision: dict[str, Any]


HookSpecificOutput = (
    PreToolUseHookSpecificOutput
    | PostToolUseHookSpecificOutput
    | PostToolUseFailureHookSpecificOutput
    | UserPromptSubmitHookSpecificOutput
    | NotificationHookSpecificOutput
    | SubagentStartHookSpecificOutput
    | PermissionRequestHookSpecificOutput
)
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

Output hook async yang menunda eksekusi hook.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Gunakan `async_` (dengan underscore) dalam kode Python. Ini secara otomatis dikonversi ke `async` ketika dikirim ke CLI.
</Note>

<h3 id="hook-usage-example">
  Contoh Penggunaan Hook
</h3>

Contoh ini mendaftarkan dua hooks: satu yang memblokir perintah bash berbahaya seperti `rm -rf /`, dan satu lagi yang mencatat semua penggunaan tool untuk audit. Hook keamanan hanya berjalan pada perintah Bash (melalui `matcher`), sementara hook logging berjalan pada semua tools.

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any


async def validate_bash_command(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data["tool_name"] == "Bash":
        command = input_data["tool_input"].get("command", "")
        if "rm -rf /" in command:
            return {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "Dangerous command blocked",
                }
            }
    return {}


async def log_tool_use(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}


options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(
                matcher="Bash", hooks=[validate_bash_command], timeout=120
            ),  # 2 min for validation
            HookMatcher(
                hooks=[log_tool_use]
            ),  # Applies to all tools (default 60s timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async for message in query(prompt="Analyze this codebase", options=options):
    print(message)
```

<h2 id="tool-input/output-types">
  Tipe Input/Output Tool
</h2>

Dokumentasi skema input/output untuk semua tools Claude Code bawaan. Meskipun Python SDK tidak mengekspor ini sebagai tipe, mereka mewakili struktur input dan output tool dalam pesan.

<h3 id="agent">
  Agent
</h3>

**Nama tool:** `Agent` (sebelumnya `Task`, yang masih diterima sebagai alias)

**Input:**

```python theme={null}
{
    "description": str,  # Deskripsi singkat tugas (3-5 kata)
    "prompt": str,  # Tugas untuk dijalankan oleh agen
    "subagent_type": str,  # Jenis agen khusus yang digunakan
}
```

**Output:**

```python theme={null}
{
    "result": str,  # Hasil akhir dari subagen
    "usage": dict | None,  # Statistik penggunaan token
    "total_cost_usd": float | None,  # Perkiraan biaya total dalam USD
    "duration_ms": int | None,  # Durasi eksekusi dalam milidetik
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nama tool:** `AskUserQuestion`

Mengajukan pertanyaan klarifikasi kepada pengguna selama eksekusi. Lihat [Handle approvals and user input](/docs/id/agent-sdk/user-input#handle-clarifying-questions) untuk detail penggunaan.

**Input:**

```python theme={null}
{
    "questions": [  # Pertanyaan untuk ditanyakan kepada pengguna (1-4 pertanyaan)
        {
            "question": str,  # Pertanyaan lengkap untuk ditanyakan kepada pengguna
            "header": str,  # Label sangat singkat ditampilkan sebagai chip/tag (maks 12 karakter)
            "options": [  # Pilihan yang tersedia (2-4 opsi)
                {
                    "label": str,  # Teks tampilan untuk opsi ini (1-5 kata)
                    "description": str,  # Penjelasan tentang arti opsi ini
                }
            ],
            "multiSelect": bool,  # Atur ke true untuk memungkinkan beberapa pilihan
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # Jawaban pengguna diisi oleh sistem izin. Jawaban multi-pilih
    # mungkin berupa daftar label atau string yang digabungkan dengan koma
}
```

**Output:**

```python theme={null}
{
    "questions": [  # Pertanyaan yang diajukan
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Memetakan teks pertanyaan ke string jawaban
    # Jawaban multi-pilih dipisahkan dengan koma
}
```

<h3 id="bash">
  Bash
</h3>

**Nama tool:** `Bash`

**Input:**

```python theme={null}
{
    "command": str,  # Perintah yang akan dijalankan
    "timeout": int | None,  # Waktu tunggu opsional dalam milidetik (maks 600000; nilai yang lebih tinggi dijepit ke maks)
    "description": str | None,  # Deskripsi jelas dan ringkas (5-10 kata)
    "run_in_background": bool | None,  # Atur ke true untuk menjalankan di latar belakang
}
```

**Output:**

```python theme={null}
{
    "output": str,  # Output stdout dan stderr gabungan
    "exitCode": int,  # Kode keluar perintah
    "killed": bool | None,  # Apakah perintah dibunuh karena waktu tunggu
    "shellId": str | None,  # ID Shell untuk proses latar belakang
}
```

<h3 id="monitor">
  Monitor
</h3>

**Nama tool:** `Monitor`

Menjalankan sumber latar belakang dan mengirimkan setiap event ke Claude sehingga dapat bereaksi tanpa polling: `command` menjalankan skrip dan mengeluarkan satu event per baris stdout, dan `ws` membuka WebSocket dan mengeluarkan satu event per frame teks. Berikan tepat satu dari `command` atau `ws`.

Ketika Monitor menjalankan perintah, ia mengikuti aturan izin yang sama seperti Bash; pengawasan WebSocket meminta persetujuan secara terpisah. Sumber `ws` memerlukan Claude Code v2.1.195 atau lebih baru. Lihat [Monitor tool reference](/docs/id/tools-reference#monitor-tool) untuk perilaku dan ketersediaan penyedia.

**Input:**

```python theme={null}
{
    "command": str | None,  # Skrip shell; setiap baris stdout adalah event, keluar mengakhiri pengawasan
    "ws": dict | None,  # Sumber WebSocket: {"url": str, "protocols": list[str] | None}; setiap frame teks adalah event
    "description": str,  # Deskripsi singkat ditampilkan dalam notifikasi
    "timeout_ms": int | None,  # Bunuh setelah batas waktu ini (default 300000, maks 3600000)
    "persistent": bool | None,  # Jalankan untuk seumur hidup sesi; hentikan dengan TaskStop
}
```

**Output:**

```python theme={null}
{
    "taskId": str,  # ID tugas monitor latar belakang
    "timeoutMs": int,  # Batas waktu dalam milidetik (0 saat persistent)
    "persistent": bool | None,  # True saat berjalan hingga TaskStop atau akhir sesi
}
```

<h3 id="edit">
  Edit
</h3>

**Nama tool:** `Edit`

**Input:**

```python theme={null}
{
    "file_path": str,  # Jalur absolut ke file yang akan dimodifikasi
    "old_string": str,  # Teks yang akan diganti
    "new_string": str,  # Teks untuk menggantinya
    "replace_all": bool | None,  # Ganti semua kemunculan (default False)
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Pesan konfirmasi
    "replacements": int,  # Jumlah penggantian yang dilakukan
    "file_path": str,  # Jalur file yang diedit
}
```

<h3 id="read">
  Read
</h3>

**Nama tool:** `Read`

**Input:**

```python theme={null}
{
    "file_path": str,  # Jalur absolut ke file yang akan dibaca
    "offset": int | None,  # Nomor baris untuk mulai membaca dari
    "limit": int | None,  # Jumlah baris yang akan dibaca
}
```

**Output (File teks):**

```python theme={null}
{
    "content": str,  # Isi file dengan nomor baris
    "total_lines": int,  # Total jumlah baris dalam file
    "lines_returned": int,  # Baris yang benar-benar dikembalikan
}
```

**Output (Gambar):**

```python theme={null}
{
    "image": str,  # Data gambar yang dikodekan Base64
    "mime_type": str,  # Tipe MIME gambar
    "file_size": int,  # Ukuran file dalam byte
}
```

<h3 id="write">
  Write
</h3>

**Nama tool:** `Write`

**Input:**

```python theme={null}
{
    "file_path": str,  # Jalur absolut ke file yang akan ditulis
    "content": str,  # Konten yang akan ditulis ke file
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Pesan sukses
    "bytes_written": int,  # Jumlah byte yang ditulis
    "file_path": str,  # Jalur file yang ditulis
}
```

<h3 id="glob">
  Glob
</h3>

**Nama tool:** `Glob`

**Input:**

```python theme={null}
{
    "pattern": str,  # Pola glob untuk mencocokkan file
    "path": str | None,  # Direktori untuk dicari (default ke cwd)
}
```

**Output:**

```python theme={null}
{
    "matches": list[str],  # Array jalur file yang cocok
    "count": int,  # Jumlah kecocokan yang ditemukan
    "search_path": str,  # Direktori pencarian yang digunakan
}
```

<h3 id="grep">
  Grep
</h3>

**Nama tool:** `Grep`

**Input:**

```python theme={null}
{
    "pattern": str,  # Pola ekspresi reguler
    "path": str | None,  # File atau direktori untuk dicari
    "glob": str | None,  # Pola glob untuk menyaring file
    "type": str | None,  # Jenis file untuk dicari
    "output_mode": str | None,  # "content", "files_with_matches", atau "count"
    "-i": bool | None,  # Pencarian tidak peka huruf besar-kecil
    "-n": bool | None,  # Tampilkan nomor baris
    "-B": int | None,  # Baris untuk ditampilkan sebelum setiap kecocokan
    "-A": int | None,  # Baris untuk ditampilkan setelah setiap kecocokan
    "-C": int | None,  # Baris untuk ditampilkan sebelum dan sesudah
    "head_limit": int | None,  # Batasi output ke N baris/entri pertama
    "multiline": bool | None,  # Aktifkan mode multiline
}
```

**Output (content mode):**

```python theme={null}
{
    "matches": [
        {
            "file": str,
            "line_number": int | None,
            "line": str,
            "before_context": list[str] | None,
            "after_context": list[str] | None,
        }
    ],
    "total_matches": int,
}
```

**Output (files\_with\_matches mode):**

```python theme={null}
{
    "files": list[str],  # File yang berisi kecocokan
    "count": int,  # Jumlah file dengan kecocokan
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nama tool:** `NotebookEdit`

**Input:**

```python theme={null}
{
    "notebook_path": str,  # Jalur absolut ke notebook Jupyter
    "cell_id": str | None,  # ID sel yang akan diedit
    "new_source": str,  # Sumber baru untuk sel
    "cell_type": "code" | "markdown" | None,  # Jenis sel
    "edit_mode": "replace" | "insert" | "delete" | None,  # Jenis operasi edit
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Pesan sukses
    "edit_type": "replaced" | "inserted" | "deleted",  # Jenis edit yang dilakukan
    "cell_id": str | None,  # ID sel yang terpengaruh
    "total_cells": int,  # Total sel dalam notebook setelah edit
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Nama tool:** `WebFetch`

**Input:**

```python theme={null}
{
    "url": str,  # URL untuk mengambil konten dari
    "prompt": str,  # Prompt untuk dijalankan pada konten yang diambil
}
```

**Output:**

```python theme={null}
{
    "bytes": int,  # Ukuran konten yang diambil dalam byte
    "code": int,  # Kode respons HTTP
    "codeText": str,  # Teks kode respons HTTP
    "result": str,  # Hasil yang diproses dari menerapkan prompt ke konten
    "durationMs": int,  # Waktu untuk mengambil dan memproses konten, dalam milidetik
    "url": str,  # URL yang diambil
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Nama tool:** `WebSearch`

**Input:**

```python theme={null}
{
    "query": str,  # Kueri pencarian yang digunakan
    "allowed_domains": list[str] | None,  # Hanya sertakan hasil dari domain ini
    "blocked_domains": list[str] | None,  # Jangan pernah sertakan hasil dari domain ini
}
```

**Output:**

```python theme={null}
{
    "query": str,  # Kueri pencarian
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Durasi pencarian dalam detik
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Nama tool:** `TodoWrite`

<Note>
  Mulai dari Claude Code v2.1.142, `TodoWrite` dinonaktifkan secara default. Gunakan `TaskCreate`, `TaskGet`, `TaskUpdate`, dan `TaskList` sebagai gantinya. Lihat [Migrate to Task tools](/docs/id/agent-sdk/todo-tracking#migrate-to-task-tools) untuk memperbarui kode pemantauan Anda, atau atur `CLAUDE_CODE_ENABLE_TASKS=0` untuk kembali ke `TodoWrite`.
</Note>

**Input:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # Deskripsi tugas
            "status": "pending" | "in_progress" | "completed",  # Status tugas
            "activeForm": str,  # Bentuk aktif deskripsi
        }
    ]
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Pesan sukses
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nama tool:** `TaskCreate`

**Input:**

```python theme={null}
{
    "subject": str,  # Judul tugas singkat
    "description": str,  # Badan tugas terperinci
    "activeForm": str | None,  # Label bentuk present-tense ditampilkan saat sedang berlangsung
    "metadata": dict | None,  # Metadata pemanggil arbitrer
}
```

**Output:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Tugas yang dibuat dengan ID yang ditugaskan
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nama tool:** `TaskUpdate`

**Input:**

```python theme={null}
{
    "taskId": str,  # ID tugas yang akan dipatch
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # ID tugas yang sekarang diblokir oleh tugas ini
    "addBlockedBy": list[str] | None,  # ID tugas yang sekarang memblokir tugas ini
    "owner": str | None,
    "metadata": dict | None,
}
```

**Output:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Nama bidang yang berubah
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Nama tool:** `TaskGet`

**Input:**

```python theme={null}
{
    "taskId": str,  # ID tugas yang akan dibaca
}
```

**Output:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None saat ID tidak ditemukan
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Nama tool:** `TaskList`

**Input:**

```python theme={null}
{}
```

**Output:**

```python theme={null}
{
    "tasks": [
        {
            "id": str,
            "subject": str,
            "status": Literal["pending", "in_progress", "completed"],
            "owner": str | None,
            "blockedBy": list[str],
        }
    ],
}
```

<h3 id="bashoutput">
  BashOutput
</h3>

**Nama tool:** `BashOutput`

**Input:**

```python theme={null}
{
    "bash_id": str,  # ID shell latar belakang
    "filter": str | None,  # Regex opsional untuk menyaring baris output
}
```

**Output:**

```python theme={null}
{
    "output": str,  # Output baru sejak pemeriksaan terakhir
    "status": "running" | "completed" | "failed",  # Status shell saat ini
    "exitCode": int | None,  # Kode keluar saat selesai
}
```

<h3 id="killbash">
  KillBash
</h3>

**Nama tool:** `KillBash`

**Input:**

```python theme={null}
{
    "shell_id": str  # ID shell latar belakang yang akan dibunuh
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Pesan sukses
    "shell_id": str,  # ID shell yang dibunuh
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nama tool:** `ExitPlanMode`

**Input:**

```python theme={null}
{
    "plan": str  # Rencana yang akan dijalankan oleh pengguna untuk persetujuan
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Pesan konfirmasi
    "approved": bool | None,  # Apakah pengguna menyetujui rencana
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nama tool:** `ListMcpResourcesTool`

**Input:**

```python theme={null}
{
    "server": str | None  # Nama server opsional untuk menyaring sumber daya
}
```

**Output:**

```python theme={null}
{
    "resources": [
        {
            "uri": str,
            "name": str,
            "description": str | None,
            "mimeType": str | None,
            "server": str,
        }
    ],
    "total": int,
}
```

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nama tool:** `ReadMcpResourceTool`

**Input:**

```python theme={null}
{
    "server": str,  # Nama server MCP
    "uri": str,  # URI sumber daya yang akan dibaca
}
```

**Output:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  Fitur Lanjutan dengan ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Membangun Antarmuka Percakapan Berkelanjutan
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio


class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions | None = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print(
            "Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session"
        )

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == "exit":
                break
            elif user_input.lower() == "interrupt":
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == "new":
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - the session retains all previous messages
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"], permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()


# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

<h3 id="using-hooks-for-behavior-modification">
  Menggunakan Hooks untuk Modifikasi Perilaku
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    HookContext,
)
import asyncio
from typing import Any


async def pre_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage before execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[PRE-TOOL] About to use: {tool_name}")

    # You can modify or block the tool execution here
    if tool_name == "Bash" and "rm -rf" in str(input_data.get("tool_input", {})):
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": "Dangerous command blocked",
            }
        }
    return {}


async def post_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log results after tool execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[POST-TOOL] Completed: {tool_name}")
    return {}


async def user_prompt_modifier(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Add context to user prompts."""
    original_prompt = input_data.get("prompt", "")

    # Add a timestamp as additional context for Claude to see
    from datetime import datetime

    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    return {
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": f"[Submitted at {timestamp}] Original prompt: {original_prompt}",
        }
    }


async def main():
    options = ClaudeAgentOptions(
        hooks={
            "PreToolUse": [
                HookMatcher(hooks=[pre_tool_logger]),
                HookMatcher(matcher="Bash", hooks=[pre_tool_logger]),
            ],
            "PostToolUse": [HookMatcher(hooks=[post_tool_logger])],
            "UserPromptSubmit": [HookMatcher(hooks=[user_prompt_modifier])],
        },
        allowed_tools=["Read", "Write", "Bash"],
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("List files in current directory")

        async for message in client.receive_response():
            # Hooks will automatically log tool usage
            pass


asyncio.run(main())
```

<h3 id="real-time-progress-monitoring">
  Pemantauan Kemajuan Real-time
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ToolUseBlock,
    ToolResultBlock,
    TextBlock,
)
import asyncio


async def monitor_progress():
    options = ClaudeAgentOptions(
        allowed_tools=["Write", "Bash"], permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Create 5 Python files with different sorting algorithms")

        # Monitor progress in real-time
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        if block.name == "Write":
                            file_path = block.input.get("file_path", "")
                            print(f"Creating: {file_path}")
                    elif isinstance(block, ToolResultBlock):
                        print("Completed tool execution")
                    elif isinstance(block, TextBlock):
                        print(f"Claude says: {block.text[:100]}...")

        print("Task completed!")


asyncio.run(monitor_progress())
```

<h2 id="example-usage">
  Contoh Penggunaan
</h2>

<h3 id="basic-file-operations-using-query">
  Operasi file dasar (menggunakan query)
</h3>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
import asyncio


async def create_project():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(
        prompt="Create a Python project structure with setup.py", options=options
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, ToolUseBlock):
                    print(f"Using tool: {block.name}")


asyncio.run(create_project())
```

<h3 id="error-handling">
  Penanganan error
</h3>

```python theme={null}
from claude_agent_sdk import query, CLINotFoundError, ProcessError, CLIJSONDecodeError

try:
    async for message in query(prompt="Hello"):
        print(message)
except CLINotFoundError:
    print(
        "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
    )
except ProcessError as e:
    print(f"Process failed with exit code: {e.exit_code}")
except CLIJSONDecodeError as e:
    print(f"Failed to parse response: {e}")
```

<h3 id="streaming-mode-with-client">
  Mode streaming dengan klien
</h3>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient
import asyncio


async def interactive_session():
    async with ClaudeSDKClient() as client:
        # Send initial message
        await client.query("What's the weather like?")

        # Process responses
        async for msg in client.receive_response():
            print(msg)

        # Send follow-up
        await client.query("Tell me more about that")

        # Process follow-up response
        async for msg in client.receive_response():
            print(msg)


asyncio.run(interactive_session())
```

<h3 id="using-custom-tools-with-claudesdkclient">
  Menggunakan tools kustom dengan ClaudeSDKClient
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    tool,
    create_sdk_mcp_server,
    AssistantMessage,
    TextBlock,
)
import asyncio
from typing import Any


# Define custom tools with @tool decorator
@tool("calculate", "Perform mathematical calculations", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        result = eval(args["expression"], {"__builtins__": {}})
        return {"content": [{"type": "text", "text": f"Result: {result}"}]}
    except Exception as e:
        return {
            "content": [{"type": "text", "text": f"Error: {str(e)}"}],
            "is_error": True,
        }


@tool("get_time", "Get current time", {})
async def get_time(args: dict[str, Any]) -> dict[str, Any]:
    from datetime import datetime

    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return {"content": [{"type": "text", "text": f"Current time: {current_time}"}]}


async def main():
    # Create SDK MCP server with custom tools
    my_server = create_sdk_mcp_server(
        name="utilities", version="1.0.0", tools=[calculate, get_time]
    )

    # Configure options with the server
    options = ClaudeAgentOptions(
        mcp_servers={"utils": my_server},
        allowed_tools=["mcp__utils__calculate", "mcp__utils__get_time"],
    )

    # Use ClaudeSDKClient for interactive tool usage
    async with ClaudeSDKClient(options=options) as client:
        await client.query("What's 123 * 456?")

        # Process calculation response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Calculation: {block.text}")

        # Follow up with time query
        await client.query("What time is it now?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Time: {block.text}")


asyncio.run(main())
```

<h2 id="sandbox-configuration">
  Konfigurasi Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Konfigurasi untuk perilaku sandbox. Gunakan ini untuk mengaktifkan sandboxing perintah dan mengonfigurasi pembatasan jaringan secara programatis.

```python theme={null}
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| Properti                    | Tipe                                                  | Default | Deskripsi                                                                                                                                                                                                                               |
| :-------------------------- | :---------------------------------------------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False` | Aktifkan mode sandbox untuk eksekusi perintah                                                                                                                                                                                           |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`  | Persetujuan otomatis perintah bash ketika sandbox diaktifkan                                                                                                                                                                            |
| `excludedCommands`          | `list[str]`                                           | `[]`    | Perintah yang selalu melewati pembatasan sandbox (misalnya, `["docker"]`). Ini berjalan tanpa sandbox secara otomatis tanpa keterlibatan model                                                                                          |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`  | Izinkan model untuk meminta menjalankan perintah di luar sandbox. Ketika `True`, model dapat mengatur `dangerouslyDisableSandbox` dalam input tool, yang jatuh kembali ke [sistem izin](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`  | Konfigurasi sandbox spesifik jaringan                                                                                                                                                                                                   |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`  | Konfigurasi pelanggaran sandbox mana yang akan diabaikan                                                                                                                                                                                |
| `enableWeakerNestedSandbox` | `bool`                                                | `False` | Aktifkan sandbox bersarang yang lebih lemah untuk kompatibilitas                                                                                                                                                                        |

<Note>
  Sandbox bergantung pada dukungan platform dan, di Linux, alat seperti `bubblewrap` dan `socat`. Secara default, ketika `enabled` adalah `True` tetapi sandbox tidak dapat dimulai, perintah berjalan tanpa sandbox dengan peringatan di stderr. Default ini berbeda dari SDK TypeScript, di mana `failIfUnavailable` default ke `true`.

  Atur `"failIfUnavailable": True` dalam pengaturan sandbox Anda untuk berhenti sebagai gantinya. Kunci belum dideklarasikan pada `SandboxSettings` namun, tetapi SDK meneruskannya ke Claude Code, yang menghormatinya. `query()` kemudian melaporkan `ResultMessage` dengan `subtype="error_during_execution"` dan alasannya dalam `errors`. Perhatikan subtype itu daripada mengharapkan `query()` untuk menaikkan sebelum menghasilkan pesan.
</Note>

<h4 id="example-usage-2">
  Contoh penggunaan
</h4>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings

sandbox_settings: SandboxSettings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "network": {"allowLocalBinding": True},
}

async for message in query(
    prompt="Build and test my project",
    options=ClaudeAgentOptions(sandbox=sandbox_settings),
):
    print(message)
```

<Warning>
  **Keamanan Unix socket**: Opsi `allowUnixSockets` dapat memberikan akses ke layanan sistem yang kuat. Misalnya, mengizinkan `/var/run/docker.sock` secara efektif memberikan akses sistem host penuh melalui API Docker, melewati isolasi sandbox. Hanya izinkan Unix sockets yang benar-benar diperlukan dan pahami implikasi keamanan dari masing-masing.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Konfigurasi spesifik jaringan untuk mode sandbox. Pengaturan ini berlaku untuk perintah Bash dalam sandbox ketika `enabled` adalah `True` dalam [`SandboxSettings`](#sandboxsettings) induk. Mereka tidak membatasi tool WebFetch, yang menggunakan [aturan izin](/docs/id/permissions#webfetch) sebagai gantinya.

```python theme={null}
class SandboxNetworkConfig(TypedDict, total=False):
    allowedDomains: list[str]
    deniedDomains: list[str]
    allowManagedDomainsOnly: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    allowLocalBinding: bool
    allowMachLookup: list[str]
    httpProxyPort: int
    socksProxyPort: int
```

| Properti                  | Tipe        | Default | Deskripsi                                                                                                                                                                             |
| :------------------------ | :---------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowedDomains`          | `list[str]` | `[]`    | Nama domain yang dapat diakses oleh proses dalam sandbox                                                                                                                              |
| `deniedDomains`           | `list[str]` | `[]`    | Nama domain yang tidak dapat diakses oleh proses dalam sandbox. Mengambil prioritas atas `allowedDomains`                                                                             |
| `allowManagedDomainsOnly` | `bool`      | `False` | Hanya pengaturan terkelola: ketika diatur dalam pengaturan terkelola, abaikan `allowedDomains` dari sumber pengaturan non-terkelola. Tidak berpengaruh ketika diatur melalui opsi SDK |
| `allowUnixSockets`        | `list[str]` | `[]`    | Jalur Unix socket yang dapat diakses proses (misalnya, Docker socket)                                                                                                                 |
| `allowAllUnixSockets`     | `bool`      | `False` | Izinkan akses ke semua Unix socket                                                                                                                                                    |
| `allowLocalBinding`       | `bool`      | `False` | Izinkan proses untuk mengikat ke port lokal (misalnya, untuk dev server)                                                                                                              |
| `allowMachLookup`         | `list[str]` | `[]`    | Hanya macOS: nama layanan XPC/Mach yang diizinkan. Mendukung wildcard di akhir                                                                                                        |
| `httpProxyPort`           | `int`       | `None`  | Port proxy HTTP untuk permintaan jaringan                                                                                                                                             |
| `socksProxyPort`          | `int`       | `None`  | Port proxy SOCKS untuk permintaan jaringan                                                                                                                                            |

<Note>
  Proxy sandbox bawaan memberlakukan daftar izin jaringan berdasarkan nama host yang diminta dan tidak menghentikan atau memeriksa lalu lintas TLS, sehingga teknik seperti [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) dapat berpotensi melewatinya. Lihat [Batasan keamanan sandboxing](/docs/id/sandboxing#security-limitations) untuk detail dan [Penyebaran aman](/docs/id/agent-sdk/secure-deployment#traffic-forwarding) untuk mengonfigurasi proxy yang menghentikan TLS.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Konfigurasi untuk mengabaikan pelanggaran sandbox tertentu.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Properti  | Tipe        | Default | Deskripsi                                     |
| :-------- | :---------- | :------ | :-------------------------------------------- |
| `file`    | `list[str]` | `[]`    | Pola jalur file untuk mengabaikan pelanggaran |
| `network` | `list[str]` | `[]`    | Pola jaringan untuk mengabaikan pelanggaran   |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback Izin untuk Perintah Tanpa Sandbox
</h3>

Ketika `allowUnsandboxedCommands` diaktifkan, model dapat meminta untuk menjalankan perintah di luar sandbox dengan mengatur `dangerouslyDisableSandbox: True` dalam input tool. Permintaan ini jatuh kembali ke sistem izin yang ada, berarti handler `can_use_tool` Anda akan dipanggil, memungkinkan Anda menerapkan logika otorisasi kustom.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Daftar statis perintah yang selalu melewati sandbox secara otomatis (misalnya, `["docker"]`). Model tidak memiliki kontrol atas ini.
  * `allowUnsandboxedCommands`: Memungkinkan model memutuskan saat runtime apakah akan meminta eksekusi tanpa sandbox dengan mengatur `dangerouslyDisableSandbox: True` dalam input tool.
</Note>

```python theme={null}
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def can_use_tool(
    tool: str, input: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model is requesting to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        if is_command_authorized(input.get("command")):
            return PermissionResultAllow()
        return PermissionResultDeny(
            message="Command not authorized for unsandboxed execution"
        )
    return PermissionResultAllow()


# Required: dummy hook keeps the stream open for can_use_tool
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Deploy my application"},
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True,  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        print(message)
```

Pola ini memungkinkan Anda untuk:

* **Audit permintaan model**: Catat ketika model meminta eksekusi tanpa sandbox
* **Implementasikan allowlist**: Hanya izinkan perintah tertentu untuk berjalan tanpa sandbox
* **Tambahkan alur persetujuan**: Memerlukan otorisasi eksplisit untuk operasi istimewa

<Warning>
  Perintah yang berjalan dengan `dangerouslyDisableSandbox: True` memiliki akses sistem penuh. Pastikan handler `can_use_tool` Anda memvalidasi permintaan ini dengan hati-hati.

  Jika `permission_mode` diatur ke `bypassPermissions` dan `allow_unsandboxed_commands` diaktifkan, model dapat secara otonom menjalankan perintah di luar sandbox tanpa prompt persetujuan apa pun. Kombinasi ini secara efektif memungkinkan model untuk melarikan diri dari isolasi sandbox secara diam-diam.
</Warning>

<h2 id="see-also">
  Lihat juga
</h2>

* [SDK overview](/docs/id/agent-sdk/overview) - Konsep SDK umum
* [TypeScript SDK reference](/docs/id/agent-sdk/typescript) - Dokumentasi SDK TypeScript
* [CLI reference](/docs/id/cli-reference) - Antarmuka baris perintah
* [Common workflows](/docs/id/common-workflows) - Panduan langkah demi langkah
