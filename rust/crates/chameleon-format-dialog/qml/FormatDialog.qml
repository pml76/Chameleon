import QtQuick
import QtQuick.Controls.FluentWinUI3

import chameleon.dialogs.format

Dialog {

    property string selectedLocale
    property string numberSignDisplay
    property string notation
    property string unitType
    property string unit

    property FormatDialogModel formatDialogModel

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
        id: localeSelector
        
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

    }

    ComboBox {
        id: numberSignDisplaySelector

        NumberSignDisplaySelectorModel {
            id: numberSignDisplaySelectorModel
        }
        
        anchors.top: localeSelector.bottom
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
        id: notationSelector

        NotionSelectorModel {
            id: notationSelectorModel
        }

        anchors.top: numberSignDisplaySelector.bottom
        anchors.topMargin: 10
        anchors.left: parent.left
        anchors.leftMargin: 10

        width: 200

        currentIndex: formatDialogModel.getNotionIndex()

        textRole: "text"
        valueRole: "value"
        model: notationSelectorModel

        onActivated: index => {
            formatDialogModel.setNotionIndex(index)
        }
    }


    ComboBox {
        id: unitTypeSelector

        UnitTypeSelectorModel {
            id: unitTypeSelectorModel
        }

        anchors.top: notationSelector.bottom
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
