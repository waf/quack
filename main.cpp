#include "rust/qt/src/Bindings.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <iostream>
using namespace std;

int main(int argc, char *argv[])
{
    cout << "main method" << endl;

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);

    QGuiApplication app(argc, argv);
    qmlRegisterType<Application>("RustCode", 1, 0, "Application");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/Main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
