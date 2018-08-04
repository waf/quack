import QtQuick 2.6
import QtQuick.Controls 2.4
import RustCode 1.0

ApplicationWindow {
    Application {
        id: application
    }

    id: window
    width: 840
    height: 520
    visible: true
    title: qsTr("Quack")

    property var pagesList  : [
        { title: "Page 1", id: "page1" },
        { title: "Page 2", id: "page2" },
        { title: "Page 3", id: "page3" },
        { title: "Page 4", id: "page4" },
        { title: "Page 5", id: "page5" }
    ];
    property string currentPage: "page1";

    header: Button {
        text: "☰"
        visible: !sidebar.persistent
        onClicked: sidebar.open()
        width: 40
        background: null
    }

    Sidebar {
        id: sidebar
        persistent: window.width > window.height
        navigationItems: pagesList
        onNavigation: currentPage = pageId
    }

    Flickable {
        id: flickable

        anchors.fill: parent
        anchors.leftMargin: sidebar.persistent ? sidebar.drawerWidth : undefined

        topMargin: 0
        bottomMargin: 0
        contentHeight: repeater.height

        Repeater {
            id: repeater
            model: pagesList;
            delegate: Loader {
                property var myModel: modelData

                id: loader
                active: false
                asynchronous: true
                anchors.fill: parent
                visible: currentPage === modelData.id
                source: "MainBar.qml"
                onVisibleChanged: loadIfNotLoaded()
                Component.onCompleted: loadIfNotLoaded()

                function loadIfNotLoaded () {
                    // to load the file at first show
                    if (visible && !active) {
                        active = true;
                    }
                }
            }
        }

        ScrollIndicator.vertical: ScrollIndicator { }
    }
}
