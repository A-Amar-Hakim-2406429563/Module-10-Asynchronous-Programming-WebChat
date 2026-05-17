# Module 10 - Tutorial 3 (WebChat using Yew)

## Experiment 3.1: Original Code

### Cara Menjalankan

1. Jalankan websocket server (bisa pake repo https://github.com/jtordgeman/SimpleWebsocketServer, terus ikutin step nya di readme nya)

2. Jalankan Yew client (repo ini):
   ```bash
   npm start
   ```

3. Buka browser:
   ```text
   http://localhost:8000
   ```

### Screenshots

![server](media/3.1-server-run.png)
![client](media/3.1-client-run.png)
![login](media/3.1-login.png)
![chat](media/3.1-chat.png)

### Reflection / Notes

Di eksperimen 3.1 ini aku jalanin server websocket dan client YewChat secara bersamaan gitu. Server jalan di port 8080, sedangkan client di port 8000. Setelah server dan client berhasil dijalankan, webchat dapat diakses lewat browser di localhost:8000. User bisa masukin username lalu masuk ke halaman chat untuk ngirim pesan realtime lewat websocket connection. Dari experiment ini aku jadi lebih ngerti gimana asynchronous programming dan websocket dipakai di aplikasi web realtime. Selain itu aku juga jadi lebih ngerti gimana Rust bisa dipakai buat frontend web application pake Yew dan WebAssembly.
