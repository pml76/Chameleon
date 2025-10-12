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

    FormatDialogModel {
        id: formatDialogModel
    }
    

    ComboBox {
        id: localeSelectorBox
        
        LocaleSelectorModel {
            id: localeSelectorModel
        }
        
        anchors.top: parent.top
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        currentIndex: formatDialogModel.getLocaleIndex()

        textRole: "text"
        valueRole: "value"
        model: localeSelectorModel
        
        onActivated: index => {
            formatDialogModel.setLocaleIndex(index)
        }
    }

    ComboBox {
        id: numberSignDisplaySelectorBox

        NumberSignDisplaySelectorModel {
            id: numberSignDisplaySelectorModel
        }
        
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


    ComboBox {
        id: unitTypeSelector

        UnitTypeSelectorModel {
            id: unitTypeSelectorModel
        }

        anchors.top: notionSelectorBox.bottom
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        //currentIndex: formatDialogModel.getNotionIndex()

        textRole: "text"
        valueRole: "value"
        model: unitTypeSelectorModel

        onActivated: index => {
            unitSelectorModel.setUnitType(currentText)
          //  formatDialogModel.setNotionIndex(index)
        }
    }

    ComboBox {
        id: unitSelector

        UnitSelectorModel {
            id: unitSelectorModel
        }

        anchors.top: unitTypeSelector.bottom
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        //currentIndex: formatDialogModel.getNotionIndex()

        textRole: "text"
        valueRole: "value"
        model: unitSelectorModel

        onActivated: index => {
            //  formatDialogModel.setNotionIndex(index)
        }
    }
}
