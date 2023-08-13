import { StyleSheet } from "react-native";

export const styles = StyleSheet.create({
  item: {
    borderColor: "darkgreen",
    borderWidth: 1,
    padding: 5,
  },
  list: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 8,
    marginTop: 5,
  },
  turn: {
    display: "flex",
    justifyContent: "center",
  },
  wrapper: {
    display: "flex",
    gap: 20,
    padding: 10,
  },
});
