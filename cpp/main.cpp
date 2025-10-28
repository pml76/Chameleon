#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

extern "C" {
    void run_main();
}

int
main(int argc, char* argv[])
{

    run_main();

    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;

    const QUrl url(
      QStringLiteral("qrc:/qt/qml/chameleon/main/qml/main.qml"));
    QObject::connect(
      &engine,
      &QQmlApplicationEngine::objectCreated,
      &app,
      [url](QObject* obj, const QUrl& objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
        },
        Qt::QueuedConnection);

    engine.load(url);

    return app.exec();
}