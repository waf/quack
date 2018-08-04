import QtQuick 2.6
import QtQuick.Controls 2.4

Item {
    id: sidebar

    signal navigation(string pageId)

    property bool persistent: true
    property var navigationItems: []
    readonly property var open: drawer.open
    readonly property alias drawerWidth: drawer.width

    Drawer {
        id: drawer

        width: 200
        height: window.height

        modal: !persistent
        interactive: !persistent
        position: !persistent ? 0 : 1
        visible: persistent

        ListView {
            id: listView
            anchors.fill: parent

            headerPositioning: ListView.OverlayHeader
            header: Pane {
                id: header
                z: 2
                width: parent.width
                visible: true

                Text {
                    text: "Quack"
                }
            }

            model: navigationItems
            delegate: ItemDelegate {
                text: modelData.title
                width: parent.width
                onClicked: sidebar.navigation(modelData.id)
            }

            ScrollIndicator.vertical: ScrollIndicator { }
        }
    }
}
