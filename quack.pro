QT += quick
CONFIG += c++11

# The following define makes your compiler emit warnings if you use
# any feature of Qt which as been marked deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
        main.cpp \
        rust/qt/src/Bindings.cpp \
        rust/qt/src/Bindings.h \
    rust/qt/src/moc_Bindings.cpp

RESOURCES += qml.qrc

# Additional import path used to resolve QML modules in Qt Creator's code model
QML_IMPORT_PATH =

# Additional import path used to resolve QML modules just for Qt Quick Designer
QML_DESIGNER_IMPORT_PATH =

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

# Link our rust library
win32:CONFIG(release, debug|release): LIBS += $$PWD/target/release/quack.dll.lib
else:win32:CONFIG(debug, debug|release): LIBS += $$PWD/target/debug/quack.dll.lib
else:unix: LIBS += -L$$PWD/target/debug/ -lquack
INCLUDEPATH += $$PWD/rust/target/debug
DEPENDPATH += $$PWD/rust/target/debug

# Change the output directory to in-tree. Cargo is also configured to write here in rust/.cargo/config
Release:DESTDIR = $$PWD/target/release
Release:OBJECTS_DIR = $$PWD/target/release/.obj
Release:MOC_DIR = $$PWD/target/release/.moc
Release:RCC_DIR = $$PWD/target/release/.rcc
Release:UI_DIR = $$PWD/target/release/.ui

Debug:DESTDIR = $$PWD/target/debug
Debug:OBJECTS_DIR = $$PWD/target/debug/.obj
Debug:MOC_DIR = $$PWD/target/debug/.moc
Debug:RCC_DIR = $$PWD/target/debug/.rcc
Debug:UI_DIR = $$PWD/target/debug/.ui
