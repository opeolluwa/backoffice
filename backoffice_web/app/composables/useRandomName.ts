import {
  uniqueNamesGenerator,
  adjectives,
  colors,
  animals,
} from "unique-names-generator";
import type { Config } from "unique-names-generator";

const customConfig: Config = {
  dictionaries: [adjectives, colors],
  separator: "-",
  length: 2,
};

export const useRandomName = (): string => {
  return uniqueNamesGenerator({
    dictionaries: [adjectives, colors, animals],
  });
};

export const useRandomShortName = (): string => {
  return uniqueNamesGenerator(customConfig);
};
