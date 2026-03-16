import { Group, GroupProps, Image } from "@mantine/core";
export type RivenGradeProps = GroupProps & {
  value: string;
  size?: string | number;
};
const IMAGE_SIZE = 25;

const GetGradeImage = (grade: string, size: string | number) => {
  switch (grade) {
    case "perfect":
      return <Image src="/grades/gradePerfect.png" h={size} w="auto" fit="contain" />;
    case "good":
      return <Image src="/grades/gradeGreen.png" h={size} w="auto" fit="contain" />;
    case "has_potential":
      return <Image src="/grades/gradeYellow.png" h={size} w="auto" fit="contain" />;
    case "bad":
      return <Image src="/grades/gradeRed.png" h={size} w="auto" fit="contain" />;
    default:
      return <Image src="/question.png" h={size} w="auto" fit="contain" />;
  }
};

export function RivenGrade({ value, size = IMAGE_SIZE, ...props }: RivenGradeProps) {
  return (
    <Group data-grade={value} {...props}>
      {GetGradeImage(value, size)}
    </Group>
  );
}
