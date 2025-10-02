# Avell Keyboard Lightning Manager

Um gerenciador de iluminação RGB para teclados de notebooks Avell, especificamente desenvolvido para o **Avell Storm 450r**. Este aplicativo permite controlar as cores do backlight do teclado e inclui a funcionalidade de captura de tela para sincronização de cores.

## Funcionalidades

-  **Controle de cores RGB**: Personalize a iluminação do seu teclado com qualquer cor
-  **Sincronização com tela**: Captura as cores dominantes da tela para sincronizar com o teclado
-  **Interface moderna**: Interface gráfica construída com React e Tauri
-  **System Tray**: Funciona em segundo plano com ícone na bandeja do sistema

## Compatibilidade

Este aplicativo foi desenvolvido especificamente para:
- **Hardware**: Avell Storm 450r e outros notebooks Avell compatíveis
- **Sistema Operacional**: Linux (testado na distribuição Manjaro 25.0.9 | Kernel 6.12.48-1-MANJARO | KDE Plasma)
- **Arquitetura**: x64

### Verificação de Compatibilidade

O aplicativo verifica automaticamente se o hardware é compatível através dos seguintes critérios:
- Presença do caminho `/sys/class/leds/rgb:kbd_backlight/multi_intensity`
- Identificação do produto como "Avell" ou "Storm 450r"
- Verificação do fabricante como "Avell"

## Pré-requisitos de Desenvolvimento

### Dependências do Sistema
As dependências do aplicativo podem ser instaladas seguindo as instruções do Tauri:
[https://v2.tauri.app/start/prerequisites/](https://v2.tauri.app/start/prerequisites/)

### Ferramentas de Desenvolvimento

#### 1. Node.js (v18 ou superior)

#### 2. Rust (v1.75 ou superior)

#### 3. Tauri CLI

## Instalação e Execução

### 1. Clonar o Repositório
```bash
git clone https://github.com/HadsonRamalho/avell-keyboard-lightning.git
cd avell-keyboard-lightning
```

### 2. Instalar Dependências
```bash
npm install
```

### 3. Executar em Modo de Desenvolvimento

⚠️ **IMPORTANTE: O aplicativo precisa ser executado com sudo** devido ao acesso necessário aos arquivos do sistema para controlar o backlight do teclado.

```bash
npm run tauri dev # Esse comando executará o aplicativo, mas as alterações não serão aplicadas no teclado.
```

### 4. Compilar para Produção
```bash
# Compilar o aplicativo
NO_STRIP=true npm run tauri build

# O executável será gerado em: src-tauri/target/release/avell-keyboard-lightning
```

### 5. Executar a Versão Compilada
```bash
# Executar o binário compilado
sudo ./src-tauri/target/release/avell-keyboard-lightning

# Ou instalar e executar
sudo cp src-tauri/target/release/avell-keyboard-lightning /usr/local/bin/
sudo avell-keyboard-lightning
```

## Como Usar

### Interface Principal
1. **Seleção de Cores**: Use o seletor de cores para escolher a iluminação desejada
2. **Prévia do Teclado**: Visualize as cores em uma representação gráfica do teclado
3. **Aplicar Cores**: As cores são aplicadas ao clicar no botão "Apply Configuration"

### Funcionalidades Adicionais
- **Captura de Tela**: Ative a sincronização automática com as cores dominantes da tela
- **System Tray**: O aplicativo pode ser minimizado para a bandeja do sistema
- **Persistência**: Continue executando em segundo plano

### Atalhos e Controles
- **Fechar**: Clique no X para enviar para a system tray (não fecha completamente)
- **Restaurar**: Clique no ícone da system tray para opções de restaurar a janela ou fechar o aplicativo

## Desenvolvimento

### Estrutura do Projeto
```
avell-keyboard-lightning/
├── src/                    # Frontend React/TypeScript
│   ├── components/         # Componentes React
│   ├── App.tsx            # Componente principal
│   └── main.tsx           # Ponto de entrada
├── src-tauri/             # Backend Rust/Tauri
│   ├── src/
│   │   ├── lightning/     # Módulo de controle do backlight
│   │   ├── lib.rs         # Biblioteca principal
│   │   └── main.rs        # Ponto de entrada
│   ├── Cargo.toml         # Dependências Rust
│   └── tauri.conf.json    # Configuração Tauri
├── package.json           # Dependências Node.js
└── README.md
```

### Comandos Úteis
```bash
# Desenvolvimento
npm run tauri dev    # Aplicação completa, sem aplicar alterações no teclado

# Build
NO_STRIP=true npm run tauri build  # Build completo

# Conceder permissão para executar o aplicativo
chmod +x ./src-tauri/target/release/avell-keyboard-lightning
```

## Solução de Problemas

### Erros Comuns

#### 1. "Permission denied" ao acessar `/sys/class/leds/`
**Solução**: Execute a aplicação com `sudo`.

#### 2. "Hardware not supported"
**Verificação**:
```bash
# Verificar se o caminho existe
ls /sys/class/leds/rgb:kbd_backlight

# Verificar informações do sistema
cat /sys/class/dmi/id/product_name
cat /sys/class/dmi/id/sys_vendor
```

#### 3. Dependências de compilação ausentes
**Solução**: Instale todas as dependências listadas na seção de pré-requisitos.

## Tecnologias Utilizadas

- **Frontend**: React 19, TypeScript, Tailwind CSS, Vite
- **Backend**: Rust, Tauri 2.0
- **UI Components**: Radix UI, ShadcnUI, Lucide React
- **Build**: Vite, Tauri CLI
- **Screen Capture**: scrap crate

## Contribuindo

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## Licença

Este projeto está licenciado sob os termos da [Licença MIT](LICENSE).

## Autor

**Hadson Ramalho** - [GitHub](https://github.com/HadsonRamalho)

---

⚡ **Nota**: Este aplicativo foi desenvolvido especificamente para notebooks Avell e requer privilégios de administrador para funcionar corretamente devido ao acesso direto aos controles de hardware do sistema.
