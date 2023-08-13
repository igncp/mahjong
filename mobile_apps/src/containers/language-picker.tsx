import React, { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Modal, Pressable, StyleSheet, Text, View } from "react-native";
import Icon from "react-native-vector-icons/FontAwesome";

const styles = StyleSheet.create({
  centeredView: {},
  modalView: {},
  row: {
    borderBottomWidth: 1,
    display: "flex",
    height: 80,
    justifyContent: "center",
    padding: 10,
  },
  textStyle: {
    fontSize: 20,
  },
  wrapper: {
    alignItems: "center",
  },
});

const Row = ({ onPress, text }: { onPress: () => void; text: string }) => (
  <Pressable onPress={onPress} style={styles.row}>
    <Text style={styles.textStyle}>{text}</Text>
  </Pressable>
);

const LanguageItem = ({
  label,
  modalVisible,
  name,
  setModalVisible,
  i18n,
}: {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  i18n: any;
  label: string;
  modalVisible: boolean;
  name: string;
  setModalVisible: (visible: boolean) => void;
}) => (
  <Row
    onPress={() => {
      i18n.changeLanguage(name);
      setModalVisible(!modalVisible);
    }}
    text={label}
  />
);

const LanguagePicker = () => {
  const { i18n, t } = useTranslation();
  const [modalVisible, setModalVisible] = useState(false);

  const languages = useMemo(
    () => [
      {
        label: t("languagePicker.en", "English"),
        name: "en",
      },
      {
        label: t("languagePicker.zh", "Chinese"),
        name: "zh",
      },
    ],
    [t]
  );

  return (
    <View style={styles.wrapper}>
      <Modal
        animationType="slide"
        onRequestClose={() => {
          setModalVisible(!modalVisible);
        }}
        transparent={false}
        visible={modalVisible}
      >
        <View style={styles.centeredView}>
          <View style={styles.modalView}>
            {languages.map((lang) => (
              <LanguageItem
                {...lang}
                i18n={i18n}
                key={lang.name}
                modalVisible={modalVisible}
                setModalVisible={setModalVisible}
              />
            ))}
            <Row
              onPress={() => setModalVisible(false)}
              text={t("languagePicker.close", "Close")}
            />
          </View>
        </View>
      </Modal>
      <Pressable onPress={() => setModalVisible(true)}>
        <Icon color="#777" name="globe" size={30} />
      </Pressable>
    </View>
  );
};

export default LanguagePicker;
