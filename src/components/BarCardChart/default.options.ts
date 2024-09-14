const GetDefaultOptions = (frontColor: string, showDatasetLabels?: boolean, tooltipShowColor?: boolean, boxWidth?: number, boxHeight?: number, tooltipCallback?: any) => {

  return {
    responsive: true,
    maintainAspectRatio: false,
    devicePixelRatio: 4,
    scale: {
      ticks: {
        precision: 0
      }
    },
    plugins: {
      legend: {
        position: "right",
        labels: {
          color: frontColor
        },
        display: (showDatasetLabels == undefined) ? false : showDatasetLabels,
      },
      tooltip: {
        displayColors: tooltipShowColor == undefined ? false : tooltipShowColor,
        boxWidth: boxWidth,
        boxHeight: boxHeight,

        callbacks: {
          ...tooltipCallback,
        }
      }
    },
    scales: {
      x: {
        grid: {
          display: true,
          drawBorder: false,
          drawOnChartArea: true,
          drawTicks: false,
          borderDash: [5, 5],
          color: "rgba(255, 255, 255, .2)",
        },
        ticks: {
          display: true,
          color: frontColor,
          padding: 10
        },
      },
      y: {
        grid: {
          drawBorder: false,
          display: true,
          drawOnChartArea: true,
          drawTicks: false,
          borderDash: [5, 5],
          color: "rgba(255, 255, 255, .2)",
        },
        ticks: {
          suggestedMin: 0,
          suggestedMax: 500,
          beginAtZero: true,
          padding: 10,
          color: frontColor,
        },
      }
    }
  }
};
export default GetDefaultOptions;