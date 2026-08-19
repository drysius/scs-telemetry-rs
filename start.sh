#!/bin/bash

# Euro Truck Simulator 2
ETS_PATH="${HOME}/.local/share/Euro Truck Simulator 2/server_config.sii"
ETS_EXEC="./bin/linux_x64/eurotrucks2_server"
ETS_APPID="1948160"

# American Truck Simulator
ATS_PATH="${HOME}/.local/share/American Truck Simulator/server_config.sii"
ATS_EXEC="./bin/linux_x64/amtrucks_server"
ATS_APPID="2239530"

# Named pipe para injeção de comandos
CMD_PIPE="/home/container/bin/server_commands"

# --- Seleção do jogo ---
if [ "$SRCDS_APPID" == "$ATS_APPID" ]; then
    OUTFILE="$ATS_PATH"
    EXEC_FILE="$ATS_EXEC"
    PROC_NAME="amtrucks_server"
    echo "[+] Configurando para American Truck Simulator (ATS)..."
elif [ "$SRCDS_APPID" == "$ETS_APPID" ]; then
    OUTFILE="$ETS_PATH"
    EXEC_FILE="$ETS_EXEC"
    PROC_NAME="eurotrucks2_server"
    echo "[+] Configurando para Euro Truck Simulator 2 (ETS2)..."
else
    OUTFILE="$ETS_PATH"
    EXEC_FILE="$ETS_EXEC"
    PROC_NAME="eurotrucks2_server"
    echo "[!] SRCDS_APPID não definido. Usando ETS2 como padrão..."
fi

if [ ! -f "$OUTFILE" ]; then
    echo "[ERRO] Arquivo de configuração '$OUTFILE' não encontrado."
    exit 1
fi

# --- Atualização de moderadores ---
if [ -n "$MODERATORS" ]; then
    IFS=',' read -r -a moderator_array <<< "$MODERATORS"
    moderator_count=${#moderator_array[@]}
    TMP_FILE=$(mktemp)

    awk -v count="$moderator_count" -v mods="$MODERATORS" '
        BEGIN { split(mods, id_array, ",") }
        /^[[:space:]]*server_config :/,/^[[:space:]]*}/ {
            if (/^[[:space:]]*moderator_list/) { next }
            if (/^[[:space:]]*}/ && !processed) {
                if (count > 0) {
                    print " moderator_list: " count
                    for (i = 0; i < count; i++) {
                        print " moderator_list[" i "]: " id_array[i+1]
                    }
                } else {
                    print " moderator_list: 0"
                }
                processed = 1
            }
        }
        { print }
    ' "$OUTFILE" > "$TMP_FILE"

    if [ $? -eq 0 ]; then
        mv "$TMP_FILE" "$OUTFILE"
        echo "[+] Lista de moderadores atualizada: $moderator_count moderador(es)."
    else
        echo "[ERRO] Falha ao atualizar moderadores."
        rm -f "$TMP_FILE"
        exit 1
    fi
else
    echo "[!] MODERATORS não definido. Pulando atualização de moderadores."
fi

# --- Cria named pipe ---
mkdir -p "$(dirname "$CMD_PIPE")"
rm -f "$CMD_PIPE"
mkfifo "$CMD_PIPE"
echo "[+] Pipe de comandos criado em: $CMD_PIPE"

# --- Inicia o servidor em background ---
echo "[+] Iniciando o servidor..."
"$EXEC_FILE" "$@" &
SERVER_PID=$!
echo "[+] Servidor PID: $SERVER_PID"

# --- Aguarda servidor inicializar antes de ligar o daemon ---
sleep 5

# --- Daemon: lê pipe e injeta no PTY do servidor ---
(
    echo "[pipe-daemon] Iniciado. Aguardando comandos em $CMD_PIPE ..."
    while true; do
        # Verifica se servidor ainda está rodando
        if ! kill -0 "$SERVER_PID" 2>/dev/null; then
            echo "[pipe-daemon] Servidor encerrado. Daemon saindo."
            break
        fi

        # Lê próximo comando do pipe (timeout 2s para checar servidor periodicamente)
        if read -r -t 2 cmd < "$CMD_PIPE" 2>/dev/null; then
            if [ -n "$cmd" ]; then
                echo "[pipe-daemon] Executando: $cmd"
                echo "$cmd" > /proc/"$SERVER_PID"/fd/0
            fi
        fi
    done
) &
DAEMON_PID=$!
echo "[+] Pipe daemon PID: $DAEMON_PID"

# --- Cleanup ao encerrar ---
cleanup() {
    echo "[+] Encerrando..."
    kill "$DAEMON_PID" 2>/dev/null
    rm -f "$CMD_PIPE"
    wait "$SERVER_PID" 2>/dev/null
}
trap cleanup EXIT SIGTERM SIGINT

# --- Aguarda servidor ---
wait "$SERVER_PID"
EXIT_CODE=$?
echo "[+] Servidor encerrado com código: $EXIT_CODE"
exit $EXIT_CODE
