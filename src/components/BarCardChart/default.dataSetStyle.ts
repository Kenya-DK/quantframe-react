const GetDefaultDatasetStyle = (frontColor: string) => {

  return {
    color: frontColor,
    backgroundColor: "rgba(255, 255, 255, 0.8)",
    label: "Default",
    tension: 0.4,
    borderWidth: 0,
    borderRadius: 4,
    data: [],
    pointRadius: 0
  }
};
export default GetDefaultDatasetStyle;