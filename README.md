# Eclipse farmer

## English

### Socials

Telegram channel: [https://t.me/fragment_software](https://t.me/fragment_software)

Telegram chat: [https://t.me/fragment_software_chat](https://t.me/fragment_software_chat)

### Donations

EVM: 0x008FA09ef36498b9731D98c1374a8FDEcC7159FC

SOL: BYvo56SfUUu6FgAiH7Q8bzFaXCnjjbabXZG6uiss25i2

### Installation

#### Prerequisites

- **Rust** : Ensure you have Rust installed. You can download and install Rust from [https://www.rust-lang.org/tools/install]().

### Build

Clone the repository and build the project:

```
git clone https://github.com/Fragment-Software/eclipse-farmer.git
cd eclipse-farmer
cargo build --release
```

### Configuration

Before running the software, configure the necessary files:

1. **evm_private_keys.txt** : Add your evm private keys to `data/evm_private_keys.txt`.
2. **eclipse_private_keys.txt** : Add your eclipse private keys to `data/eclipse_private_keys.txt`.
3. **proxies.txt** : Add your proxies to `data/proxies.txt`. Both http and socks5 are supported in the following format: http://log:pass@ip:port or socks5://log:pass@ip:port

### Running

Execute the built binary:

`cargo run --release`

### Disclaimer

All materials and software are provided "as is" without any warranties. I am not responsible for any direct or indirect damages resulting from the use or inability to use this software.

## Русский

### Наши ресурсы

Telegram channel: [https://t.me/fragment_software](https://t.me/fragment_software)

Telegram chat: [https://t.me/fragment_software_chat](https://t.me/fragment_software_chat)

### Донаты

EVM: 0x008FA09ef36498b9731D98c1374a8FDEcC7159FC

SOL: BYvo56SfUUu6FgAiH7Q8bzFaXCnjjbabXZG6uiss25i2

### Установка

#### Предварительные требования

- **Rust** : Убедитесь, что Rust установлен. Вы можете скачать и установить Rust с [https://www.rust-lang.org/tools/install]().

### Сборка

Клонируйте репозиторий и соберите проект:

```
git clone https://github.com/Fragment-Software/eclipse-farmer.git
cd eclipse-farmer
cargo build --release

```

### Конфигурация

Перед запуском программного обеспечения настройте необходимые файлы:

1. **evm_private_keys.txt** : Добавьте ваши EVM приватные ключи в `data/evm_private_keys.txt`.
2. **eclipse_private_keys.txt** : Добавьте ваши Eclipse приватные ключи в `data/eclipse_private_keys.txt`.
3. **proxies.txt** : Добавьте ваши прокси в `data/proxies.txt`. Поддерживаются как http, так и socks5. Формат: http://log:pass@ip:port либо socks5://log:pass@ip:port

### Запуск

Запустите собранный бинарный файл:

`cargo run --release `

### Дисклеймер

Все материалы и программное обеспечение предоставляются "как есть" без каких-либо гарантий. Я не несу ответственности за любые прямые или косвенные убытки, возникшие в результате использования или невозможности использования данного программного обеспечения.
