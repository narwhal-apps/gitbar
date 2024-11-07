export interface ValidatorResult {
  [validatorName: string]: {
    error: boolean;
    message?: string;
  };
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type ValidatorFn = (value: any) => ValidatorResult;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function required(value: any): ValidatorResult {
  if (value === '' || value == null) {
    return { required: { error: true, message: 'Field is required' } };
  }
  return { required: { error: false } };
}

export const Validators = {
  required,
};
