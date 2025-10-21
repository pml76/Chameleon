import QtQuick
import QtQuick.Controls.FluentWinUI3
import QtQuick.Window

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import chameleon.main
import chameleon.dialogs.format

ApplicationWindow {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640
    color: palette.window

    DataFrameModel {
        id: tableModel
    }

    FormatDialog {
        id: dialog

        selectedLocale: formatDialogModel.defaultLocale()
        numberSignDisplay: formatDialogModel.defaultNumberSignDisplay()
        notation: formatDialogModel.defaultNotation()
        unitType: formatDialogModel.defaultUnitType()
        unit: formatDialogModel.defaultUnit()
    }

    Button {
        id: button
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right

        text: "Open Dialog"

        onClicked: dialog.open()
    }

    Rectangle {

        anchors.top: button.bottom
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right

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
}
