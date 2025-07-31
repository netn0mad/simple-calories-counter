#include <iostream>
#include "sqlite3.h"  // Заголовок SQLite

int main() {
    sqlite3 *db;
    int rc = sqlite3_open(":memory:", &db);  // Открываем БД в памяти
    
    if (rc != SQLITE_OK) {
        std::cerr << "Ошибка открытия БД: " << sqlite3_errmsg(db) << std::endl;
        return 1;
    }
    
    std::cout << "SQLite подключен! Версия: " << sqlite3_libversion() << std::endl;
    sqlite3_close(db);
    return 0;
}