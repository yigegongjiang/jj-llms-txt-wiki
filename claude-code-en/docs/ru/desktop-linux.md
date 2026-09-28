> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop на Linux (бета)

> Установка и обновление приложения Claude Desktop на Ubuntu и Debian

<Note>
  Поддержка Linux для приложения Claude Desktop находится в бета-версии.
</Note>

Приложение Desktop на Linux предоставляет вам тот же опыт Chat, Cowork и Claude Code, что и на macOS и Windows: параллельные сеансы, визуальный просмотр различий, интегрированный терминал и редактор, а также предпросмотр приложения в реальном времени. Полный справочник функций см. в разделе [Use Claude Code Desktop](/docs/ru/desktop).

<h2 id="requirements">
  Требования
</h2>

* Дистрибутив на основе Debian: Ubuntu 22.04 или более поздняя версия, или Debian 12 или более поздняя версия
* x86\_64 или arm64

Другие дистрибутивы на основе Debian, соответствующие этим требованиям, могут работать, но официально не тестируются. На дистрибутивах, которые не основаны на Debian, таких как Fedora или Arch, запустите [CLI](/docs/ru/setup#system-requirements) вместо этого. Если вы работаете на Windows с WSL 2, установите приложение Windows Desktop и запускайте сеансы внутри вашего дистрибутива; см. [Claude Code Desktop в WSL](/docs/ru/desktop-wsl).

<h3 id="cowork-requirements">
  Требования Cowork
</h3>

Cowork — это вкладка рабочего стола для [Dispatch и более длительной агентивной работы](https://claude.com/docs/cowork/overview). На Linux Cowork запускает эти задачи на виртуальной машине, которую приложение рабочего стола размещает с помощью QEMU и KVM. Для использования Cowork ваша машина должна иметь:

* **Аппаратная виртуализация**: включена в параметрах прошивки. Без неё вкладка Cowork сообщает «Cowork requires hardware virtualization (KVM)».
* **QEMU и прошивка UEFI**: `qemu-system-x86`, `ovmf` и `virtiofsd` на x86\_64, или `qemu-system-arm`, `qemu-efi-aarch64` и `virtiofsd` на arm64. `apt install claude-desktop` устанавливает их по умолчанию как рекомендуемые пакеты. Если вы установили с флагом `--no-install-recommends`, или ваша система — это минимальный образ, который пропускает рекомендуемые пакеты, вкладка Cowork сообщает «Cowork requires QEMU» и показывает команду `apt install` для запуска. Ubuntu 22.04 не имеет пакета `virtiofsd`; приложение использует там встроенную копию.
* **Доступ к `/dev/kvm`**: добавьте вашего пользователя в группу `kvm` с помощью `sudo usermod -aG kvm $USER`, затем выйдите и снова войдите. Некоторые окружения рабочего стола предоставляют вошедшему пользователю доступ к `/dev/kvm` без группы, но Cowork также требует `/dev/vhost-vsock`, который могут открыть только члены группы `kvm`. Присоединитесь к группе, даже если `/dev/kvm` уже работает для вас.

Приложение проверяет эти требования один раз при запуске: перезагрузите его после установки пакетов и выйдите, а затем снова войдите после присоединения к группе. Если `/dev/vhost-vsock` отсутствует и ваше работающее ядро не имеет каталога модулей в `/lib/modules`, вкладка Cowork сообщает, что ядро не включает поддержку виртуализации, которая требуется Cowork, и что она не может быть добавлена вручную. Эта комбинация распространена на ChromeOS и в контейнерных окружениях Linux.

<h2 id="install">
  Установка
</h2>

Установите из репозитория apt Anthropic, чтобы обновления поступали через обычные обновления пакетов вашей системы. Откройте терминал и выполните команды на каждом шаге.

<Steps>
  <Step title="Добавьте репозиторий apt Anthropic">
    Этот шаг загружает ключ подписи с помощью `curl` и проверяет его с помощью `gpg`, которые свежие установки Debian и Ubuntu могут не включать. Если какая-либо команда сообщает `command not found`, сначала установите оба:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Загрузите ключ подписи Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Команда ничего не выводит при успехе и выводит ошибку `curl:` при неудаче. Отсутствующий или неправильный ключ приводит к сбою `apt update` позже с `NO_PUBKEY BAA929FF1A7ECACE`, поэтому подтвердите, что ключ загружен и принадлежит Anthropic, прежде чем продолжить:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    Отпечаток, который выводит gpg, должен быть `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`. Если gpg сообщает, что файл не может быть открыт или не содержит действительных данных OpenPGP, загрузка не удалась или вернула неправильное содержимое: подтвердите, что ваша сеть может достичь `downloads.claude.ai`, затем повторно выполните команду загрузки.

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

<h3 id="install-from-a-downloaded-file">
  Установка из загруженного файла
</h3>

Если вы не можете установить через репозиторий apt, загрузите пакет `.deb` непосредственно из пула пакетов репозитория. Эта команда ищет самый новый пакет для вашей архитектуры в индексе репозитория, а затем загружает его в текущий каталог:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Если команда завершится с ошибкой `Remote file name has no length`, поиск не вернул путь к пакету. Это может означать, что индекс репозитория не удалось получить, например, когда ваша сеть блокирует `downloads.claude.ai`, или что пакет не существует для вашей архитектуры. Подтвердите, что ваша сеть может достичь `downloads.claude.ai` и что `dpkg --print-architecture` выводит `amd64` или `arm64`; репозиторий не публикует пакеты для других архитектур.

Чтобы установить без регистрации репозитория apt Anthropic, сначала создайте `/etc/default/claude-desktop` со строкой `CLAUDE_DESKTOP_ADD_REPO="false"`. Без репозитория apt не доставляет новые версии; для обновления повторно выполните команду загрузки и переустановите, или [зарегистрируйте репозиторий](#install) позже.

Затем откройте загруженный файл с помощью установщика программного обеспечения, такого как GNOME Software, или установите его с помощью apt из каталога, содержащего загруженный файл:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Если apt сообщает об ошибке `E: Unsupported file ./claude-desktop_*.deb given on commandline`, шаблон не совпадает с файлом `.deb` в текущем каталоге. Подтвердите, что загрузка завершена, затем выполните команду снова из каталога, содержащего файл.

Установка `.deb` также регистрирует репозиторий apt Anthropic в `/etc/apt/sources.list.d/claude-desktop.list`, поэтому будущие обновления поступают с [обычными обновлениями пакетов](#update) вашей системы.

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

Удаление пакета также удаляет запись репозитория и ключ подписи, которые он зарегистрировал. Если вы добавили запись репозитория самостоятельно на этапе [Добавление apt репозитория Anthropic](#install), удалите её также:

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

* Выполните `sudo apt update` после добавления репозитория. `apt install` сам по себе не видит репозиторий, который вы добавили после последнего запуска `apt update`.
* Подтвердите, что запись репозитория была записана. `cat /etc/apt/sources.list.d/claude-desktop.list` должна показать строку `deb` из шага [Add Anthropic's apt repository](#install). Если файл пуст или отсутствует, выполните этот шаг снова.
* Подтвердите, что ваша архитектура поддерживается. `dpkg --print-architecture` должна вывести `amd64` или `arm64`. Репозиторий не публикует пакеты для других архитектур.
* Выполните `sudo apt update` снова и проверьте его вывод на наличие ошибок, связанных с `downloads.claude.ai`. Ошибка сети или ключа там означает, что репозиторий был добавлен, но не может быть достигнут или проверен.

Если репозиторий на месте и доступен, а пакет все еще не найден, вместо этого [install from a downloaded file](#install-from-a-downloaded-file).

<h3 id="unmet-dependencies">
  Unmet dependencies
</h3>

Если `apt` останавливается с сообщением `The following packages have unmet dependencies` или `Unsatisfied dependencies`, прочитайте, какую зависимость он называет:

* `libc6 (>= 2.34)`: ваш дистрибутив старше, чем поддерживает пакет. Ubuntu 20.04 поставляется с `libc6` 2.31. Обновитесь до Ubuntu 22.04 или позже, или Debian 12 или позже.
* Все отсутствующие зависимости показывают `not installable` с суффиксом `:amd64` или `:arm64`: вы загрузили `.deb` для другой архитектуры, чем архитектура вашей машины. Выполните `dpkg --print-architecture` и загрузите соответствующий `.deb`, или [установите из репозитория apt](#install), который выбирает пакет для вашей архитектуры.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  Running as root without --no-sandbox is not supported
</h3>

Если `claude-desktop` завершается с этим сообщением, вы запустили его от имени root. Войдите как обычный пользователь и запустите его оттуда.

<h3 id="cowork-isn’t-available">
  Cowork isn't available
</h3>

Если вкладка Cowork показывает одно из этих сообщений, исправьте требование, которое оно называет, затем перезагрузите приложение:

* **Cowork requires QEMU**: установите [пакеты QEMU и UEFI firmware](#cowork-requirements), которые указаны в сообщении.
* **Cowork requires hardware virtualization (KVM)**: включите [аппаратную виртуализацию](#cowork-requirements) в параметрах прошивки.
* **Claude doesn't have permission to use virtualization (/dev/kvm)**: добавьте вашего пользователя в [группу `kvm`](#cowork-requirements), затем выйдите и войдите снова.
* **Cowork requires the `vhost_vsock` kernel module**: выполните `sudo modprobe vhost_vsock`, затем перезагрузите приложение. Это загружает модуль только для текущей загрузки. Чтобы загружать его при каждой загрузке, выполните `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Что еще не включено в бета-версию Linux
</h2>

* **Computer Use**: [управление приложением и экраном](/docs/ru/desktop#let-claude-use-your-computer) недоступно на Linux.
* **Dictation**: голосовой ввод недоступен в приложении Claude Desktop для Linux. Используйте [голосовую диктовку](/docs/ru/voice-dictation) в CLI вместо этого.
* **Quick Entry global hotkey**: работает на X11. На нативном Wayland требуется портал GlobalShortcuts вашей среды рабочего стола.
* **Fedora и RHEL**: сегодня поддерживаются только дистрибутивы на основе Debian. Поддержка дополнительных дистрибутивов появится в будущем.

Для всего, что еще недоступно в приложении Desktop, [CLI](/docs/ru/quickstart) запускает тот же механизм Claude Code и поддерживает более широкий диапазон дистрибутивов Linux; см. [системные требования](/docs/ru/setup#system-requirements).
