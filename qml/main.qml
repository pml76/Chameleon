import QtQuick
import QtQuick.Controls
import QtQuick.Window

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640
    color: palette.window

    MyObject {
        id: myObject
        number: 1
        string: qsTr("My String with my number: %1").arg(myObject.number)
    }

    DataFrameModel {
        id: tableModel
    }


    Rectangle {

        anchors.fill: parent

        HorizontalHeaderView {
            id: horizontalHeader
            anchors.left: tableView.left
            anchors.top: parent.top
            syncView: tableView
            clip: true
        }

        VerticalHeaderView {
            id: verticalHeader
            anchors.top: tableView.top
            anchors.left: parent.left
            syncView: tableView
            clip: true
        }

        TableView {
            id: tableView
            anchors.left: verticalHeader.right
            anchors.top: horizontalHeader.bottom
            anchors.right: parent.right
            anchors.bottom: parent.bottom

            columnSpacing: 1
            rowSpacing: 1
            clip: true

            model: tableModel

            delegate: Rectangle {
                implicitWidth: 100
                implicitHeight: 50
                Text {
                    anchors.centerIn: parent
                    text: display
                }
            }
        }
    }

    /*
    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(myObject.number)
            color: palette.text
        }

        Label {
            text: qsTr("String: %1").arg(myObject.string)
            color: palette.text
        }

        Button {
            text: qsTr("Increment Number")

            onClicked: myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")

            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }

        Button {
            text: qsTr("Quit")

            onClicked: Qt.quit()
        }

    }
    */
}
