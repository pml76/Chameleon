import QtQuick
import QtQuick.Controls.FluentWinUI3

import chameleon.dialogs.format

Dialog {

    x: 100
    y: 100
    width: 800
    height: 400

    title: "Title"
    standardButtons: Dialog.Ok | Dialog.Cancel
    popupType: Popup.Window

    modal: true
    dim: true

    onAccepted: {
        numberSignDisplaySelectorModel.selectedNumberSignDisplay = numberSignDisplaySelectorBox.currentValue
        console.log(" numberSignDisplaySelectorModel.selectedNumberSignDisplay set to: ", numberSignDisplaySelectorBox.currentText)
    }

    FormatDialogModel {
        id: formatDialogModel
    }

    LocaleSelectorModel {
        id: localeSelectorModel
    }

    NumberSignDisplaySelectorModel {
        id: numberSignDisplaySelectorModel
    }

    ComboBox {
        id: localeSelectorBox

        anchors.top: parent.top
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        currentIndex: localeSelectorModel.getCurrentIndex()

        textRole: "text"
        valueRole: "value"
        model: localeSelectorModel
    }

    ComboBox {
        id: numberSignDisplaySelectorBox

        anchors.top: localeSelectorBox.bottom
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        textRole: "text"
        valueRole: "value"
        model: numberSignDisplaySelectorModel

        currentIndex: formatDialogModel.getNumberSignDisplayIndex()

        onActivated: index => {
            formatDialogModel.setNumberSignDisplayIndex(index)
        }
    }

    ComboBox {
        id: notionSelectorBox

        NotionSelectorModel {
            id: notionSelectorModel
        }

        anchors.top: numberSignDisplaySelectorBox.bottom
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        currentIndex: formatDialogModel.getNotionIndex()

        textRole: "text"
        valueRole: "value"
        model: notionSelectorModel

        onActivated: index => {
            formatDialogModel.setNotionIndex(index)
        }
    }
}
