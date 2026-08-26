> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop на Linux (бета)

> Установка и обновление приложения Claude Desktop на Ubuntu и Debian

<Note>
  Поддержка Linux для приложения Claude Desktop находится в бета-версии. Доступны все вкладки Chat, Cowork и Code.
</Note>

Приложение Desktop на Linux предоставляет вам тот же опыт Chat, Cowork и Claude Code, что и на macOS и Windows: параллельные сеансы, визуальный просмотр различий, интегрированный терминал и редактор, а также предпросмотр приложения в реальном времени. Полный справочник функций см. в разделе [Use Claude Code Desktop](/docs/ru/desktop).

<h2 id="requirements">
  Требования
</h2>

* Ubuntu 22.04 или более поздняя версия, или Debian 12 или более поздняя версия
* x86\_64 или arm64

Другие дистрибутивы на основе Debian, соответствующие этим требованиям, могут работать, но официально не тестируются.

<h2 id="install">
  Установка
</h2>

Установите из репозитория apt Anthropic, чтобы обновления поступали через обычные обновления пакетов вашей системы. Откройте терминал и выполните команды на каждом шаге.

<Steps>
  <Step title="Добавьте репозиторий apt Anthropic">
    Этот шаг загружает ключ подписи с помощью `curl`, который свежие установки Debian и Ubuntu могут не включать. Если команда загрузки завершится с ошибкой `sudo: curl: command not found`, сначала установите curl:

    ```bash theme={null}
    sudo apt install curl
    ```

    Загрузите ключ подписи Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Зарегистрируйте репозиторий:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Установите пакет">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Запустите и войдите">
    Запустите **Claude** из вашего средства запуска приложений или выполните `claude-desktop` из терминала и войдите с помощью вашей учетной записи Anthropic.

    Приложение Linux входит так же, как на macOS и Windows: с подпиской claude.ai или через SSO вашей организации. Desktop не принимает ключ API Claude Console напрямую; используйте [CLI](/docs/ru/quickstart) для аутентификации по ключу API. Для корпоративных развертываний, которые маршрутизируют Desktop на Agent Platform Google Cloud или шлюз LLM, см. [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) и [конфигурация сети](/docs/ru/network-config).
  </Step>
</Steps>

<Accordion title="Проверьте ключ подписи">
  Вы можете подтвердить, что загруженный ключ подписи принадлежит Anthropic:

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  Отпечаток должен быть `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Установка из загруженного файла
</h3>

Если вы не можете установить через репозиторий apt, загрузите пакет `.deb` непосредственно из пула пакетов репозитория. Эта команда ищет самый новый пакет для вашей архитектуры в индексе репозитория, а затем загружает его в текущий каталог:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Если команда завершится с ошибкой `Remote file name has no length`, поиск не вернул путь к пакету. Это может означать, что индекс репозитория не удалось получить, например, когда ваша сеть блокирует `downloads.claude.ai`, или что пакет не существует для вашей архитектуры. Подтвердите, что ваша сеть может достичь `downloads.claude.ai` и что `dpkg --print-architecture` выводит `amd64` или `arm64`; репозиторий не публикует пакеты для других архитектур.

Затем откройте загруженный файл с помощью установщика программного обеспечения, такого как GNOME Software, или установите его с помощью apt из каталога, содержащего загруженный файл:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Если apt сообщает об ошибке `E: Unsupported file ./claude-desktop_*.deb given on commandline`, шаблон не совпадает с файлом `.deb` в текущем каталоге. Подтвердите, что загрузка завершена, затем выполните команду снова из каталога, содержащего файл.

Пакет `.deb`, установленный таким образом, не получает обновления. Чтобы получать обновления через apt, зарегистрируйте репозиторий из шага [Добавьте репозиторий apt Anthropic](#install). Пакет также записывает закомментированную запись репозитория в `/etc/apt/sources.list.d/claude-desktop.list`; раскомментирование его строки `deb` эквивалентно.

<h2 id="update">
  Обновление
</h2>

Приложение Desktop не обновляется само по себе на Linux. Обновления поступают с обычными обновлениями пакетов вашей системы:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

Графический обновитель программного обеспечения вашего дистрибутива также будет подхватывать новые версии.

<h2 id="uninstall">
  Удаление
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

Это удаляет ключ подписи вместе с приложением, поэтому если вы добавили запись репозитория во время установки, удалите и её:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Troubleshooting
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  Unable to locate package claude-desktop
</h3>

Если `sudo apt install claude-desktop` завершается с ошибкой `E: Unable to locate package claude-desktop`, apt не смог найти добавленный вами репозиторий. Проверьте следующее:

* Подтвердите, что запись репозитория была записана. `cat /etc/apt/sources.list.d/claude-desktop.list` должна показать строку `deb` из шага [Add Anthropic's apt repository](#install). Если файл пуст или отсутствует, выполните этот шаг снова.
* Подтвердите, что ваша архитектура поддерживается. `dpkg --print-architecture` должна вывести `amd64` или `arm64`. Репозиторий не публикует пакеты для других архитектур.
* Выполните `sudo apt update` снова и проверьте его вывод на наличие ошибок, связанных с `downloads.claude.ai`. Ошибка сети или ключа там означает, что репозиторий был добавлен, но не может быть достигнут или проверен.

Если репозиторий на месте и доступен, а пакет все еще не найден, вместо этого [install from a downloaded file](#install-from-a-downloaded-file).

<h2 id="what’s-not-in-the-linux-beta-yet">
  Что еще не включено в бета-версию Linux
</h2>

* **Computer Use**: [управление приложением и экраном](/docs/ru/desktop#let-claude-use-your-computer) недоступно на Linux.
* **Dictation**: голосовой ввод недоступен в приложении Claude Desktop для Linux. Используйте [голосовую диктовку](/docs/ru/voice-dictation) в CLI вместо этого.
* **Quick Entry global hotkey**: работает на X11. На нативном Wayland требуется портал GlobalShortcuts вашей среды рабочего стола.
* **Fedora и RHEL**: сегодня поддерживаются только дистрибутивы на основе Debian. Поддержка дополнительных дистрибутивов появится в будущем.

Для всего, что еще недоступно в приложении Desktop, [CLI](/docs/ru/quickstart) запускает тот же механизм Claude Code и поддерживает более широкий диапазон дистрибутивов Linux; см. [системные требования](/docs/ru/setup#system-requirements).
