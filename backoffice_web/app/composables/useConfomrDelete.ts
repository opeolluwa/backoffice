import { AppConfirmDelete } from "#components";

export function useConfirmDelete() {
  const overlay = useOverlay();

  return (props?: {
    title?: string;
    description?: string;
  }) =>
    new Promise<boolean>((resolve) => {
      const modal = overlay.create(AppConfirmDelete);

      const instance = modal.open({ props: { ...props, modelValue: true } });

      instance.on("confirm", () => resolve(true));
      instance.on("close", () => resolve(false));
    });
}