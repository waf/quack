import QtQuick 2.6
import QtQuick.Controls 2.4

Column {
    id: column
    spacing: 10
    anchors.margins: 10
    anchors.left: parent.left
    anchors.right: parent.right

    Label {
        font.pixelSize: 22
        width: parent.width
        elide: Label.ElideRight
        horizontalAlignment: Qt.AlignHCenter
        text: myModel.title
    }

    Label {
        width: parent.width
        wrapMode: Label.WordWrap
        text: qsTr("This example demonstrates how Drawer can be used as a non-closable persistent side panel.\n\n" +
                   "When the application is in portrait mode, the drawer is an interactive side panel that can " +
                   "be swiped open from the left edge. When the application is in landscape mode, the drawer " +
                   "and the content are laid out side by side.")
    }
}
